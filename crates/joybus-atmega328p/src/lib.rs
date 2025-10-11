#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

#[cfg(feature = "joybus")]
mod joybus;
#[cfg(feature = "joybus")]
pub use joybus::new_console;

#[cfg(feature = "ufmt")]
use ufmt::derive::uDebug;

mod protocol;
pub use protocol::{read_bytes, send_byte, ReadError};
