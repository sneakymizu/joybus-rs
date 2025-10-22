#![no_std]

#[cfg(feature="atmega328p")]
pub use joybus_rs_atmega328p::new_console;

pub use joybus_rs_core::*;