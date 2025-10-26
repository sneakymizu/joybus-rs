use arduino_hal::port::{
    mode::{Floating, Input},
    Pin, PinOps,
};
use joybus_rs_core::{JoybusConsole, JoybusError};

use crate::{timer::TimerConfigurator, ReadError};
mod boilerplate;
use boilerplate::{JoybusPinRead, JoybusPinWrapper};

pub(crate) type AvrPinDefinition<PIN> = Pin<Input<Floating>, PIN>;

pub struct JoybusPin<PIN: PinOps, TIMER> {
    pin: JoybusPinWrapper<PIN, TIMER>,
}

pub fn new_console<PIN: PinOps, TIMER>(
    pin: AvrPinDefinition<PIN>,
    mut timer: TIMER,
) -> JoybusPin<PIN, TIMER>
where
    TIMER: TimerConfigurator,
{
    timer.configure_count_cycles();
    timer.configure_timeout(64);
    JoybusPin {
        pin: JoybusPinWrapper::from_pin_and_timer(pin, timer),
    }
}

impl<PIN: PinOps, TIMER> JoybusConsole for JoybusPin<PIN, TIMER>
where
    JoybusPinWrapper<PIN, TIMER>: JoybusPinRead,
{
    fn read_write(
        &mut self,
        write_data: &[u8],
        read_data: &mut [u8],
    ) -> Result<usize, JoybusError> {
        self.pin
            .joybus_read(write_data, read_data)
            .map_err(|e| match e {
                ReadError::OutOfMemory(bytes) => {
                    JoybusError::OutOfMemory(bytes as usize)
                }
                ReadError::Timeout(timeout_value) => {
                    JoybusError::Timeout(timeout_value as usize)
                }
                ReadError::UnknownError(error_value) => {
                    JoybusError::ImplementationReportsError(error_value as usize)
                }
            })
    }
}
