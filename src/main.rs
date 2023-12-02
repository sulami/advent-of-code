#![no_std]
#![no_main]

use core::fmt::Write;

use arduino_nano33iot::{
    self as bsp,
    hal::{
        clock::GenericClockController,
        delay::Delay,
        pac::{interrupt, CorePeripherals, Peripherals},
        prelude::*,
        usb::UsbBus,
    },
    Led, Pins,
};
use arrayvec::{ArrayString, ArrayVec};
use cortex_m::peripheral::NVIC;
use panic_halt as _;
use usb_device::{bus::UsbBusAllocator, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[cfg(feature = "day-01")]
mod day_01;
#[cfg(feature = "day-02")]
mod day_02;

#[arduino_nano33iot::entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = Pins::new(peripherals.PORT);
    let mut led: Led = pins.led_sck.into();
    let mut usb = unsafe {
        UsbClient::new(
            &mut clocks,
            peripherals.USB,
            &mut peripherals.PM,
            pins.usb_dm,
            pins.usb_dp,
            &mut core,
        )
    };
    let mut delay = Delay::new(core.SYST, &mut clocks);

    delay.delay_ms(500_u16);
    usb.write(b"Getting started...\n");

    #[cfg(feature = "day-01")]
    let (p1, p2) = day_01::solve();
    #[cfg(feature = "day-02")]
    let (p1, p2) = day_02::solve();

    let mut s = ArrayString::<127>::new();
    writeln!(&mut s, "Part 1: {}\nPart 2: {}", p1, p2).unwrap();

    loop {
        delay.delay_ms(1_000_u16);
        led.toggle().unwrap();
        usb.poll(&noop);
        usb.write(s.as_bytes());
    }
}

fn noop(_: &[u8]) {}

/// The USB allocator for the USB client.
static mut BUS_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
/// The USB bus.
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
/// The USB-based serial port.
static mut USB_SERIAL: Option<SerialPort<UsbBus, [u8; 128], [u8; 1024]>> = None;
/// Internal buffer of received serial messages.
static mut SERIAL_BUFFER: ArrayVec<[u8; 127], 10> = ArrayVec::new_const();

/// A USB logger that can be used to send data to the host.
struct UsbClient {}

impl UsbClient {
    /// Create a new USB client.
    unsafe fn new(
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

    fn write(&mut self, bytes: &[u8]) {
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

    fn poll<T>(&mut self, handler: &impl Fn(&[u8]) -> T) -> Option<T> {
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
