#![no_std]
#![no_main]

use embassy_executor::{main, Spawner};

const READ_COMMAND: u16 = 0b11;
const READ_COMMAND_LENGTH: u8 = 9;
const STATE_RESPONSE_LENGTH: u8 = 32;

#[main]
fn main(spawner: Spawner) {
    loop {}
}
