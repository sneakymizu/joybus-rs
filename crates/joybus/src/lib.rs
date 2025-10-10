#![no_std]

#[cfg(feature = "print_error")]
use ufmt::derive::uDebug;

#[cfg_attr(feature = "print_error", derive(uDebug))]
pub enum JoybusError {
    Timeout,
    ResponseMismatch,
    OutOfMemory(usize),
}
pub trait JoybusConsole {
    fn read_write(&mut self, write_data: &[u8], read_data: &mut [u8])
        -> Result<usize, JoybusError>;
}
impl<T: JoybusConsole> JoybusConsoleExt for T {}
pub trait JoybusConsoleExt: JoybusConsole {
    fn read_contoller_state_alloc(&mut self) -> Result<JoybusControllerState, JoybusError> {
        let mut data = JoybusControllerState([0u8; 4]);
        self.read_contoller_state(&mut data)?;
        Ok(data)
    }
    fn read_contoller_state(
        &mut self,
        data: &mut JoybusControllerState,
    ) -> Result<(), JoybusError> {
        let res = self.read_write(&[commands::POLL_SIGNAL], &mut data.0)?;
        if res != 4 {
            Err(JoybusError::ResponseMismatch)
        } else {
            Ok(())
        }
    }
}

#[derive(Default)]
#[cfg_attr(feature = "print_error", derive(uDebug))]
pub struct JoybusControllerState([u8; 4]);
impl From<[u8; 4]> for JoybusControllerState {
    fn from(value: [u8; 4]) -> Self {
        JoybusControllerState(value)
    }
}
impl From<u32> for JoybusControllerState {
    fn from(value: u32) -> Self {
        JoybusControllerState(value.to_le_bytes())
    }
}
impl JoybusControllerState {
    pub fn a_button(&self) -> bool {
        (self.0[0] & 128u8) != 0
    }
    pub fn b_button(&self) -> bool {
        (self.0[0] & 64u8) != 0
    }
    pub fn z_button(&self) -> bool {
        (self.0[0] & 32u8) != 0
    }
    pub fn start_button(&self) -> bool {
        (self.0[0] & 16u8) != 0
    }
    pub fn dpad_up(&self) -> bool {
        (self.0[0] & 8u8) != 0
    }
    pub fn dpad_down(&self) -> bool {
        (self.0[0] & 4u8) != 0
    }
    pub fn dpad_left(&self) -> bool {
        (self.0[0] & 2u8) != 0
    }
    pub fn dpad_right(&self) -> bool {
        (self.0[0] & 1u8) != 0
    }
    pub fn reset(&self) -> bool {
        (self.0[1] & 128u8) != 0
    }
    pub fn left_trigger(&self) -> bool {
        (self.0[1] & 32u8) != 0
    }
    pub fn right_trigger(&self) -> bool {
        (self.0[1] & 16u8) != 0
    }
    pub fn c_up(&self) -> bool {
        (self.0[1] & 8u8) != 0
    }
    pub fn c_down(&self) -> bool {
        (self.0[1] & 4u8) != 0
    }
    pub fn c_left(&self) -> bool {
        (self.0[1] & 2u8) != 0
    }
    pub fn c_right(&self) -> bool {
        (self.0[1] & 1u8) != 0
    }
    pub fn x_axis(&self) -> i8 {
        self.0[2] as i8
    }
    pub fn y_axis(&self) -> i8 {
        self.0[3] as i8
    }
}

pub mod commands {
    pub const POLL_SIGNAL: u8 = 0b00000001u8;
}
