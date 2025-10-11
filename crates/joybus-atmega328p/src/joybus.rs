use arduino_hal::port::{
    mode::{Floating, Input},
    Pin, PinOps,
};
use joybus_rs::JoybusConsole;

use crate::ReadError;
mod boilerplate;
use boilerplate::{JoybusPinRead, JoybusPinWrapper, TimerConfigurator};

pub(crate) type AvrPinDefinition<PIN> = Pin<Input<Floating>, PIN>;

pub struct JoybusPin<PIN: PinOps, TIMER> {
    pin: JoybusPinWrapper<PIN, TIMER>,
}

#[allow(private_bounds)]
pub fn new_console<PIN: PinOps, TIMER>(
    pin: AvrPinDefinition<PIN>,
    mut timer: TIMER,
) -> JoybusPin<PIN, TIMER>
where
    TIMER: TimerConfigurator, // this trait should only be implemented internally, so this should only check against internal implementations
{
    timer.configure();
    JoybusPin {
        pin: JoybusPinWrapper::from_pin_and_timer(pin, timer),
    }
}

impl TimerConfigurator for ::arduino_hal::pac::TC0 {
    fn configure(&mut self) {
        // setup timer for joybus readings
        // normal operating timer
        self.tccr0a.reset();
        self.tccr0b.write(|w| w.cs0().direct()); // no prescale, normal timer operation
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
