use core::any::{Any, TypeId};

use arduino_hal::{
    hal::port::{self, Dynamic, PB1},
    port::{
        mode::{Floating, Input, Io, Output},
        Pin, PinMode, PinOps,
    },
};
use joybus_rs::JoybusConsole;

use crate::{read_bytes, send_byte, ReadError};

pub struct JoybusPin<PIN> {
    pin: PIN,
}

impl<PIN> JoybusPin<PIN> {
    pub fn from_pin(input: PIN) -> Self {
        Self { pin: input }
    }
}

struct PinWrapper<PIN: PinOps>{
    input: Option<Pin<Input<Floating>, PIN>>,
    output: Option<Pin<Output, PIN>>
}
impl<PIN: PinOps> PinWrapper<PIN>{
    pub fn from_pin(pin:Pin<Input<Floating>, PIN>)->Self{
        Self { input: Some(pin), output: None }
    }
}

trait JoybusPinRead<const R: usize, const W: usize> {
    fn joybus_read(&mut self, send: &[u8; W], recv: &mut [u8; R])->Result<usize,ReadError>;
}

impl<const R: usize, const W: usize> JoybusPinRead<R, W> for PinWrapper<port::PB0> {
    fn joybus_read(&mut self, send: &[u8; W], recv: &mut [u8; R]) -> Result<usize, ReadError>{
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x00, W>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x0, 0x26, 0x15, 1, R>(recv) }.map(|v| v as usize)
    }
}

impl<const R: usize, const W: usize> JoybusPinRead<R, W> for PinWrapper<port::PB1> {
    fn joybus_read(&mut self, send: &[u8; W], recv: &mut [u8; R]) -> Result<usize, ReadError>{
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x01, W>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x1, 0x26, 0x15, 1, R>(recv) }.map(|v| v as usize)
    }
}

impl<const R: usize, const W: usize> JoybusPinRead<R, W> for PinWrapper<port::PB2> {
    fn joybus_read(&mut self, send: &[u8; W], recv: &mut [u8; R]) -> Result<usize, ReadError>{
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x02, W>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x2, 0x26, 0x15, 1, R>(recv) }.map(|v| v as usize)
    }
}

impl<const R: usize, const W: usize> JoybusPinRead<R, W> for PinWrapper<port::PB3> {
    fn joybus_read(&mut self, send: &[u8; W], recv: &mut [u8; R]) -> Result<usize, ReadError>{
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x03, W>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x3, 0x26, 0x15, 1, R>(recv) }.map(|v| v as usize)
    }
}

impl<const R: usize, const W: usize> JoybusPinRead<R, W> for PinWrapper<port::PB4> {
    fn joybus_read(&mut self, send: &[u8; W], recv: &mut [u8; R]) -> Result<usize, ReadError>{
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x04, W>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x4, 0x26, 0x15, 1, R>(recv) }.map(|v| v as usize)
    }
}
impl<const R: usize, const W: usize> JoybusPinRead<R, W> for PinWrapper<port::PB5> {
    fn joybus_read(&mut self, send: &[u8; W], recv: &mut [u8; R]) -> Result<usize, ReadError>{
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x05, W>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x5, 0x26, 0x15, 1, R>(recv) }.map(|v| v as usize)
    }
}
impl<const R: usize, const W: usize> JoybusPinRead<R, W> for PinWrapper<port::PB6> {
    fn joybus_read(&mut self, send: &[u8; W], recv: &mut [u8; R]) -> Result<usize, ReadError>{
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x06, W>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x6, 0x26, 0x15, 1, R>(recv) }.map(|v| v as usize)
    }
}
impl<const R: usize, const W: usize> JoybusPinRead<R, W> for PinWrapper<port::PB7> {
    fn joybus_read(&mut self, send: &[u8; W], recv: &mut [u8; R]) -> Result<usize, ReadError>{
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x07, W>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x7, 0x26, 0x15, 1, R>(recv) }.map(|v| v as usize)
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
