use arduino_nano33iot::{
    self as bsp,
    hal::{
        clock::GenericClockController,
        pac::{interrupt, CorePeripherals},
        usb::UsbBus,
    },
};
use arrayvec::ArrayVec;
use cortex_m::peripheral::NVIC;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

/// The USB allocator for the USB client.
static mut BUS_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
/// The USB bus.
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
/// The USB-based serial port.
static mut USB_SERIAL: Option<SerialPort<UsbBus, [u8; 128], [u8; 1024]>> = None;
/// Internal buffer of received serial messages.
static mut SERIAL_BUFFER: ArrayVec<[u8; 127], 10> = ArrayVec::new_const();

/// A USB logger that can be used to send data to the host.
pub struct UsbClient {}

impl UsbClient {
    /// Create a new USB client.
    pub unsafe fn new(
        clocks: &mut GenericClockController,
        usb: bsp::pac::USB,
        pm: &mut bsp::pac::PM,
        usb_dm: impl Into<bsp::UsbDm>,
        usb_dp: impl Into<bsp::UsbDp>,
        core: &mut CorePeripherals,
    ) -> Self {
        // Setup an 8 MHz clock for USB.
        let usb_clock_src = clocks.gclk0();
        let usb_clock = clocks.usb(&usb_clock_src).unwrap();

        BUS_ALLOCATOR = Some(UsbBusAllocator::new(UsbBus::new(
            &usb_clock,
            pm,
            usb_dm.into(),
            usb_dp.into(),
            usb,
        )));

        let bus_allocator = BUS_ALLOCATOR.as_ref().unwrap();

        USB_SERIAL = Some(SerialPort::new_with_store(
            bus_allocator,
            [0u8; 128],
            [0u8; 1024],
        ));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x2222, 0x3333))
                .manufacturer("Fractal Dynamics")
                .product("Advent of Code 2023")
                .serial_number("AOC2023")
                .device_class(USB_CLASS_CDC)
                .build(),
        );

        // Setup interrupts.
        core.NVIC.set_priority(interrupt::USB, 1);
        NVIC::unmask(interrupt::USB);

        Self {}
    }

    pub fn write(&mut self, bytes: &[u8]) {
        unsafe {
            if let Some(serial) = USB_SERIAL.as_mut() {
                let mut start = 0;
                // Write out all the bytes.
                loop {
                    cortex_m::interrupt::free(|_| {
                        if let Ok(bytes_written) = serial.write(&bytes[start..]) {
                            start += bytes_written;
                        }
                    });
                    if start == bytes.len() {
                        break;
                    }
                }
                // Ensure we flush all of them.
                loop {
                    if cortex_m::interrupt::free(|_| serial.flush().is_ok()) {
                        break;
                    }
                }
            }
        }
    }

    pub fn poll<T>(&mut self, handler: &impl Fn(&[u8]) -> T) -> Option<T> {
        unsafe { SERIAL_BUFFER.pop().map(|buf| handler(&buf)) }
    }
}

/// The hardware USB interrupt, which triggers on time to be compliant
/// with the USB standard, regardless of whether there might be other
/// computation ongoing.
#[interrupt]
unsafe fn USB() {
    if let Some(bus) = USB_BUS.as_mut() {
        if let Some(serial) = USB_SERIAL.as_mut() {
            if !bus.poll(&mut [serial]) {
                return;
            }
            let mut buf = [0u8; 127];
            let _ = serial
                .read(&mut buf)
                .map(|_byte_read| SERIAL_BUFFER.try_insert(0, buf));
        }
    }
}
