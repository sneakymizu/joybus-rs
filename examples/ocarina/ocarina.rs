#![no_std]
#![no_main]

use arduino_hal::{clock::Clock, delay_ms};
use panic_halt as _;
use ufmt::{derive::uDebug, uwriteln};

use joybus_rs_atmega328p::new_console;

use joybus_rs::{JoybusConsoleExt, JoybusControllerState};

use crate::notes::{BaseNote, BaseNoteSelection, Note, PwmOcarina};

mod notes;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();

    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut reader_pin = new_console(pins.d6, dp.TC0);
    let mut pwm_ocarina = PwmOcarina::from_timer(pins.d9.into_output(), dp.TC1);

    led_pin.set_low();
    let mut n64_controller_state = JoybusControllerState::default();

    let mut currently_selected_note: BaseNote;
    loop {
        delay_ms(100);
        match reader_pin.read_contoller_state(&mut n64_controller_state) {
            Ok(_) => (),
            Err(e) => {
                let _ = uwriteln!(serial, "Got error {:?}\r", e);
                continue;
            }
        };

        let note_selection: BaseNoteSelection = (&n64_controller_state).into();
        let note: Result<BaseNote, u8> = note_selection.try_into();
        currently_selected_note = match note {
            Ok(note) => note,
            Err(0) => {
                let _ = uwriteln!(serial, "Nothing selected\r");
                continue;
            }
            Err(e) => {
                let _ = uwriteln!(
                    serial,
                    "Simultaniously selected notes (counting {}), not switching.\r",
                    e
                );
                continue;
            }
        };
        pwm_ocarina.play_sound::<440>(notes::Sound::Vibrato(
            currently_selected_note.into(),
            n64_controller_state.x_axis().abs(),
        ));
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
