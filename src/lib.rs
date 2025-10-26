#![no_std]

#[cfg(feature = "atmega328p")]
pub use joybus_rs_atmega328p::new_console as new_atmega328p_console;

pub use joybus_rs_core as core;

pub mod prelude {
    pub use joybus_rs_core::*;
}
