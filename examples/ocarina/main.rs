#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::{default_serial, delay_ms};
use joybus_rs_atmega328p::new_console;

use joybus_rs_core::{JoybusConsoleExt, JoybusControllerState};
use ufmt::uwriteln;

use crate::notes::{Note, Step};
use crate::ocarina::{NoteSelectionError, OcarinaNote, OcarinaNoteSelection, PwmOcarina};

mod notes;
mod ocarina;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    let mut serial = default_serial!(dp, pins, 57600);
    let mut reader_pin = new_console(pins.d6, dp.TC0);
    let mut pwm_ocarina = PwmOcarina::from_timer(pins.d9.into_output(), dp.TC1);

    led_pin.set_low();
    let mut n64_controller_state = JoybusControllerState::default();

    loop {
        // there seem to be issues if this is polling too fast resulting in more
        // timeout errors as the polling signal gets resent too fast
        // therefore a small delay here.
        delay_ms(50);
        match reader_pin.read_contoller_state(&mut n64_controller_state) {
            Ok(_) => (),
            Err(e) => {
                let _ = uwriteln!(serial, "Some when reading controller state {:?}", e);
                continue;
            }
        }

        let note: Result<Note, NoteSelectionError> = (&n64_controller_state).try_into();
        match note {
            Ok(note) => {
                let sound = ocarina::Sound::Modulation(
                    note
                    - if n64_controller_state.z_button(){Step::HalfStep}else{Step::None}  // augments half step down
                    + if n64_controller_state.right_trigger(){Step::HalfStep}else{Step::None} // augments half step up
                    + n64_controller_state.y_axis().signum() * Step::FullStep, // augments a whole step
                    n64_controller_state.x_axis(),
                );
                pwm_ocarina.play_sound::<440>(sound);
                led_pin.set_low();
            }
            Err(NoteSelectionError::NoNoteToPlay) => {
                pwm_ocarina.play_sound::<440>(ocarina::Sound::Nothing);
                led_pin.set_low();
            }
            Err(NoteSelectionError::TooManyNotesToPlay(amount)) => {
                let _ = uwriteln!(serial, "Cannot play {} notes at the same time.", amount);
                led_pin.set_high();
            }
        };
    }
}

impl From<&JoybusControllerState> for OcarinaNoteSelection {
    fn from(value: &JoybusControllerState) -> Self {
        let note_selections = (value.c_right() as u8) << OcarinaNote::A5 as u8
            | (value.c_left() as u8) << OcarinaNote::B5 as u8
            | (value.a_button() as u8) << OcarinaNote::D5 as u8
            | (value.c_down() as u8) << OcarinaNote::F5 as u8
            | (value.c_up() as u8) << OcarinaNote::D6 as u8;
        Self::from_bitmask(note_selections)
    }
}
impl TryFrom<&JoybusControllerState> for Note {
    type Error = NoteSelectionError;

    fn try_from(value: &JoybusControllerState) -> Result<Self, Self::Error> {
        let selection: OcarinaNoteSelection = value.into();
        let base_note: OcarinaNote = selection.try_into()?;
        Ok(base_note.into())
    }
}
