use core::any::{Any, TypeId};

use arduino_hal::{
    hal::port::{self, Dynamic, PB1},
    port::{
        mode::{Io, Output},
        Pin, PinMode, PinOps,
    },
};
use joybus_rs::JoybusConsole;

use crate::{read_bytes, send_byte};

pub struct JoybusPin<PIN> {
    pin: PIN,
}

impl<PIN> JoybusPin<PIN> {
    pub fn from_pin(input: PIN) -> Self {
        Self { pin: input }
    }
}

trait Read<const R: usize, const W: usize> {
    fn read(&mut self, send: &[u8; W], recv: &mut [u8; R]);
}
impl<const R: usize, const W: usize> Read<R, W> for Pin<Dynamic, port::PB0> {
    fn read(&mut self, send: &[u8; W], recv: &mut [u8; R]) {
        unsafe {
            self.make_output();
        };
        unsafe { send_byte::<0x0b, 0x06, W>(send) };
        self.into_input();
        let _ = unsafe { read_bytes::<0x9, 0x6, 0x26, 0x15, 1, R>(recv) };
    }
}

impl<PIN> JoybusConsole for JoybusPin<PIN> {
    fn read<const COMMAND: u8>(
        &mut self,
        data: &mut [u8],
    ) -> Result<usize, joybus_rs::JoybusError> {
        unsafe { send_byte::<0x0b, 0x06, 1>(&[joybus_rs::commands::POLL_SIGNAL]) };
        let _ = unsafe { read_bytes::<0x9, 0x6, 0x26, 0x15, 1, 4>(data) };
        Ok(0)
    }

    fn write<const COMMAND: u8>(
        &mut self,
        write_data: &[u8],
        read_data: &mut [u8],
    ) -> Result<usize, joybus_rs::JoybusError> {
        todo!()
    }
}
