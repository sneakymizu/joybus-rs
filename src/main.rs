#![no_std]
#![no_main]

use esp_hal::{gpio, main, peripherals::Peripherals};
use ufmt::uwriteln;

const READ_COMMAND: u16 = 0b11;
const READ_COMMAND_LENGTH: u8 = 9;
const STATE_RESPONSE_LENGTH: u8 = 32;

#[main]
fn main() -> ! {
    let peripherals = unsafe { Peripherals::steal() };
    let pin = gpio::Flex::new(peripherals.GPIO16);
    pin.wait_for_falling_edge();
}
