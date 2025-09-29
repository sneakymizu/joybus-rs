use embedded_hal::digital::{InputPin, OutputPin};
use joybus_rs::JoybusConsole;

use crate::{read_bytes, send_byte};

enum SwappablePin<L, R> {
    Left(L),
    Right(R),
}
impl<L, R> SwappablePin<L, R> {
    fn left(&mut self) -> L {
        match self {
            SwappablePin::Left(l) => l,
            SwappablePin::Right(_r) => panic!("Right was unassigned but you tried to access it."),
        }
    }
    fn right(&mut self) -> R {
        match self {
            SwappablePin::Left(_l) => panic!("Left was unassigned but you tried to access it."),
            SwappablePin::Right(r) => r,
        }
    }
}
trait SwappablePinQuestionMark {
    fn into_output();
    fn into_input();
    fn get_port();
    fn get_pin();
}

struct JoybusPin<I: InputPin, O: OutputPin> {
    pin: SwappablePin<I, O>,
}

impl<I: InputPin, O: OutputPin> JoybusPin<I, O> {
    fn from_input(input: I) -> Self {
        Self {
            pin: SwappablePin::Left(input),
        }
    }
    fn from_output(output: O) -> Self {
        Self {
            pin: SwappablePin::Right(output),
        }
    }
}

impl<I: InputPin, O: OutputPin> JoybusConsole for JoybusPin<I, O> {
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
