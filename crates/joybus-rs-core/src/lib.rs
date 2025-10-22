#![no_std]

#[cfg(feature = "ufmt")]
use ufmt::derive::uDebug;

/// Provides error variants that might occur during Joybus interactions
#[cfg_attr(feature = "ufmt", derive(uDebug))]
pub enum JoybusError {
    /// If a timeout occurs during reading (e.g. line does not go to low) this error provides the timeout in clock cycles.
    Timeout(usize),
    /// If the response did not match expectations - e.g. too litle bytes were read from a controller - a ResponseMismatch is returned.
    /// The value contained in ResponseMismatch reflects the bytes read instead.
    ResponseMismatch(usize),
    /// If too little memory was supplied to the read function, an OutOfMemory error will be returned containing the amount of cycles since the line was pulled low.
    /// This should be used to debug timing issues with the stop bit.
    OutOfMemory(usize),
    /// If any implementation chooses to provide an implementation specific error code the ImplementationReportsError can be used with the error value.
    ImplementationReportsError(usize),
}

/// The JoybusConsole trait describes methods to be implemented when implementing the Joybus-Protocol for a participant from the console side.
pub trait JoybusConsole {
    /// The read_write method writes the given bytes on the line and reads the response into read_data. For any errors that might occur refer to [JoybusError].
    ///
    /// # Arguments
    ///
    /// * `write_data`: Byte array to write.
    /// * `read_data`: Byte array to read bytes into
    ///
    /// # Returns
    ///
    /// * The result should be either the amount of bytes written to `read_data` on success or an Err-value describing the error as a [JoybusError]
    fn read_write(&mut self, write_data: &[u8], read_data: &mut [u8])
        -> Result<usize, JoybusError>;
}

impl<T: JoybusConsole> JoybusConsoleExt for T {}

/// The [JoybusConsoleExt] trait provides standard command implementations for any [JoybusConsole].
pub trait JoybusConsoleExt: JoybusConsole {
    /// Reads the currently attached contoller state into a newly alocated [JoybusControllerState] structure.
    fn read_contoller_state_alloc(&mut self) -> Result<JoybusControllerState, JoybusError> {
        let mut data = JoybusControllerState([0u8; 4]);
        self.read_contoller_state(&mut data)?;
        Ok(data)
    }
    /// Reads the currently attached contoller state into the given [JoybusControllerState] structure.
    fn read_contoller_state(
        &mut self,
        data: &mut JoybusControllerState,
    ) -> Result<(), JoybusError> {
        let res = self.read_write(&[commands::POLL_SIGNAL], &mut data.0)?;
        if res != 4 {
            Err(JoybusError::ResponseMismatch(res))
        } else {
            Ok(())
        }
    }
}

/// The [JoybusControllerState] stores the information of the currently read controller's state.
/// It can be used to check whether the attached controller currently provides inputs.
#[derive(Default)]
#[cfg_attr(feature = "ufmt", derive(uDebug))]
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
    /// True if the N64-Controllers-A-button is pressed.
    pub fn a_button(&self) -> bool {
        (self.0[0] & 128u8) != 0
    }
    /// True if the N64-Controllers-B-button is pressed.
    pub fn b_button(&self) -> bool {
        (self.0[0] & 64u8) != 0
    }
    /// True if the N64-Controllers-Z-button is pressed.
    pub fn z_button(&self) -> bool {
        (self.0[0] & 32u8) != 0
    }
    /// True if the N64-Controllers-Start-button is pressed.
    pub fn start_button(&self) -> bool {
        (self.0[0] & 16u8) != 0
    }
    /// True if the N64-Controllers-D-Pad-Up-button is pressed.
    pub fn dpad_up(&self) -> bool {
        (self.0[0] & 8u8) != 0
    }
    /// True if the N64-Controllers-D-Pad-Down-button is pressed.
    pub fn dpad_down(&self) -> bool {
        (self.0[0] & 4u8) != 0
    }
    /// True if the N64-Controllers-D-Pad-Left-button is pressed.
    pub fn dpad_left(&self) -> bool {
        (self.0[0] & 2u8) != 0
    }
    /// True if the N64-Controllers-D-Pad-Right-button is pressed.
    pub fn dpad_right(&self) -> bool {
        (self.0[0] & 1u8) != 0
    }
    /// True if the N64-Controllers-Reset-button is pressed.
    pub fn reset(&self) -> bool {
        (self.0[1] & 128u8) != 0
    }
    /// True if the N64-Controllers-Left-Shoulder-button is pressed.
    pub fn left_trigger(&self) -> bool {
        (self.0[1] & 32u8) != 0
    }
    /// True if the N64-Controllers-Right-Shoulder-button is pressed.
    pub fn right_trigger(&self) -> bool {
        (self.0[1] & 16u8) != 0
    }
    /// True if the N64-Controllers-C-Up-button is pressed.
    pub fn c_up(&self) -> bool {
        (self.0[1] & 8u8) != 0
    }
    /// True if the N64-Controllers-C-Down-button is pressed.
    pub fn c_down(&self) -> bool {
        (self.0[1] & 4u8) != 0
    }
    /// True if the N64-Controllers-C-Left-button is pressed.
    pub fn c_left(&self) -> bool {
        (self.0[1] & 2u8) != 0
    }
    /// True if the N64-Controllers-C-Right-button is pressed.
    pub fn c_right(&self) -> bool {
        (self.0[1] & 1u8) != 0
    }
    /// Provides the displacement value of the N64-Controllers-Analogue-Stick in x (horizontal) direction.
    /// Positive if the Analogue-Stick is tilted right. Negative if it is tilted left. Interval goes from [-128,127].
    pub fn x_axis(&self) -> i8 {
        self.0[2] as i8
    }
    /// Provides the displacement value of the N64-Controllers-Analogue-Stick in y (vertical) direction.
    /// Positive if the Analogue-Stick is tilted up. Negative if it is tilted down. Interval goes from [-128,127].
    pub fn y_axis(&self) -> i8 {
        self.0[3] as i8
    }
}

/// Provides all known Joybus commands.
pub mod commands {
    #[cfg(doc)]
    use super::*;

    /// Used to poll the controllers state. See [JoybusConsoleExt::read_contoller_state] and [JoybusControllerState].
    pub const POLL_SIGNAL: u8 = 0b00000001u8;
}
