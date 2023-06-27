#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;
use ufmt::uwriteln;

use n64_controller::N64ControllerConnection;

mod n64_controller;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut read_write_pin = N64ControllerConnection::from_pin(pins.d6.into_output(), &dp.TC0);

    uwriteln!(serial, "Lets go").void_unwrap();
    led_pin.set_low();
    loop {
        let res = read_write_pin.send_recv(0b11);
        match res {
            Ok(res) => uwriteln!(serial, "Should read {}", res).void_unwrap(),
            Err(_) => uwriteln!(serial, "Failed reading").void_unwrap(),
        };
        arduino_hal::delay_ms(100);
    }
}
