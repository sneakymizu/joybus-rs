#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use panic_halt as _;

use crate::atmega328p::send_byte;

mod atmega328p;
mod n64;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    //let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut _reader_pin = pins.d6.into_output_high();

    //uwriteln!(serial, "Lets go\r").unwrap();
    led_pin.set_low();
    arduino_hal::delay_ms(3000);
    send_byte::<0x0b, 0x06>(n64::commands::POLL_SIGNAL);
    _reader_pin.into_pull_up_input();
    loop {
        arduino_hal::delay_ms(100);
    }
}
