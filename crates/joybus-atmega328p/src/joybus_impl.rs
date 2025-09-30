use core::marker::PhantomData;

use embedded_hal::digital::{InputPin, OutputPin};
use joybus_rs::JoybusConsole;

use crate::{read_bytes, send_byte};

pub struct OutputPinData{
    pub pin_number: u8,
    pub port: u8,
}
pub struct InputPinData{
    pub pin_number: u8,
    pub pin:u8
}
pub trait SwappablePin {
    fn as_output(&mut self)->&OutputPinData;
    fn as_input(&mut self)->&InputPinData;
}

pub struct JoybusPin<P: SwappablePin> {
    pin: P,
}

impl<P: SwappablePin> JoybusPin<P> {
    pub fn from_pin(pin: P) -> Self {
        Self {
            pin,
        }
    }
}

impl<P: SwappablePin> JoybusConsole for JoybusPin<P> {
    fn read<const COMMAND: u8>(
        &mut self,
        data: &mut [u8],
    ) -> Result<usize, joybus_rs::JoybusError> {
        match self.pin.as_output(){
            OutputPinData { pin_number, port }=>unsafe {
                send_byte::<0x0b, 0x06, 1>([joybus_rs::commands::POLL_SIGNAL])
            }
        }
        unsafe { send_byte::<0x0b, 0x06, 1>([joybus_rs::commands::POLL_SIGNAL]) };
        self.pin.as_input();
        let _ = unsafe { read_bytes::<0x9, 0x6, 0x26, 0x15, 1, 4>(data) };
        self.pin.as_output();
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
