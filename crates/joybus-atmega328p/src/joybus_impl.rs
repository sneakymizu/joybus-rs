use joybus_rs::JoybusConsole;

use crate::{read_bytes, send_byte};

struct JoybusPin<PIN> {
    pin: PIN,
}

impl<PIN> JoybusPin<PIN> {
    pub fn from_pin(input: PIN) -> Self {
        Self { pin: input }
    }
}

macro_rules! console_read {
    ($port:item) => {};
}

impl<PIN> JoybusConsole for JoybusPin<PIN> {
    fn read<const COMMAND: u8>(
        &mut self,
        data: &mut [u8],
    ) -> Result<usize, joybus_rs::JoybusError> {
        unsafe { send_byte::<0x0b, 0x06, 1>([joybus_rs::commands::POLL_SIGNAL]) };
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
