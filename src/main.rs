#![no_std]
#![no_main]

use core::fmt::Write;

use arduino_nano33iot::{
    hal::{
        clock::GenericClockController,
        delay::Delay,
        pac::{CorePeripherals, Peripherals},
        prelude::*,
    },
    Led, Pins,
};
use arrayvec::ArrayString;
use panic_halt as _;

mod serial;

#[cfg(feature = "day-01")]
mod day_01;
#[cfg(feature = "day-02")]
mod day_02;
#[cfg(feature = "day-03")]
mod day_03;

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
        serial::UsbClient::new(
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

    let (p1, p2) = (0, 0);
    #[cfg(feature = "day-01")]
    let (p1, p2) = day_01::solve();
    #[cfg(feature = "day-02")]
    let (p1, p2) = day_02::solve();
    #[cfg(feature = "day-03")]
    let (p1, p2) = day_03::solve();

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
