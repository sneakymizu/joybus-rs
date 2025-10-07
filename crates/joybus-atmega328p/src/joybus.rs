use arduino_hal::port::PinOps;
use joybus_rs::JoybusConsole;

use crate::{
    joybus::boilerplate::{JoybusPinWrapper, JoybusPinWrapping},
    ReadError,
};

mod boilerplate;

trait JoybusPinRead {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError>;
}

pub struct JoybusPin<PIN: PinOps> {
    pin: JoybusPinWrapper<PIN>,
}
pub fn new_console<PIN: PinOps>(pin: JoybusPinWrapping<PIN>) -> JoybusPin<PIN> {
    JoybusPin {
        pin: JoybusPinWrapper::from_pin(pin),
    }
}

impl<PIN: PinOps> JoybusConsole for JoybusPin<PIN>
where
    JoybusPinWrapper<PIN>: JoybusPinRead,
{
    fn read_write(
        &mut self,
        write_data: &[u8],
        read_data: &mut [u8],
    ) -> Result<usize, joybus_rs::JoybusError> {
        self.pin
            .joybus_read(write_data, read_data)
            .map_err(|e| match e {
                ReadError::OutOfMemory(bytes) => {
                    joybus_rs::JoybusError::OutOfMemory(bytes as usize)
                }
                ReadError::Timeout(_) => joybus_rs::JoybusError::Timeout,
                ReadError::UnknownError(_) => joybus_rs::JoybusError::ResponseMismatch,
            })
    }
}
