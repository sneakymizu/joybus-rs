#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::delay_ms;
use joybus_rs_atmega328p::new_console;

use joybus_rs::{JoybusConsoleExt, JoybusControllerState};

use crate::notes::{BaseNote, BaseNoteSelection, Note, NoteSelectionError, PwmOcarina};

mod notes;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    let mut reader_pin = new_console(pins.d6, dp.TC0);
    let mut pwm_ocarina = PwmOcarina::from_timer(pins.d9.into_output(), dp.TC1);

    led_pin.set_low();
    let mut n64_controller_state = JoybusControllerState::default();

    loop {
        delay_ms(100);
        let Ok(_) = reader_pin.read_contoller_state(&mut n64_controller_state) else {
            continue;
        };

        let note: Result<Note, NoteSelectionError> = (&n64_controller_state).try_into();
        match note {
            Ok(note) => {
                pwm_ocarina.play_sound::<440>(notes::Sound::Modulation(
                    note
                    - n64_controller_state.z_button() as i8  // augments half step down
                    + n64_controller_state.right_trigger() as i8 // augments half step up
                    + n64_controller_state.y_axis().signum() * 2, // augments a whole step
                    n64_controller_state.x_axis().abs(),
                ));
                led_pin.set_low();
            }
            Err(NoteSelectionError::NoNoteToPlay) => {
                pwm_ocarina.play_sound::<440>(notes::Sound::Nothing);
                led_pin.set_low();
            }
            Err(NoteSelectionError::TooManyNotesToPlay(amount)) => {
                let _ = uwriteln!(serial, "Cannot play {} notes at the same time.", amount);
                led_pin.set_high();
            }
        };
    }
}

impl From<&JoybusControllerState> for BaseNoteSelection {
    fn from(value: &JoybusControllerState) -> Self {
        let note_selections = (value.c_right() as u8) << BaseNote::A5 as u8
            | (value.c_left() as u8) << BaseNote::B5 as u8
            | (value.a_button() as u8) << BaseNote::D5 as u8
            | (value.c_down() as u8) << BaseNote::F5 as u8
            | (value.c_up() as u8) << BaseNote::D6 as u8;
        Self::from_notes(note_selections)
    }
}
impl TryFrom<&JoybusControllerState> for Note {
    type Error = NoteSelectionError;

    fn try_from(value: &JoybusControllerState) -> Result<Self, Self::Error> {
        let selection: BaseNoteSelection = value.into();
        let base_note: BaseNote = selection.try_into()?;
        Ok(base_note.into())
    }
}
