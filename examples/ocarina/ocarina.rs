#![no_std]
#![no_main]

use arduino_hal::{clock::Clock, delay_ms};
use panic_halt as _;
use ufmt::{derive::uDebug, uwriteln};

use joybus_rs_atmega328p::new_console;

use joybus_rs::{JoybusConsoleExt, JoybusControllerState};

use crate::notes::{
    frequency_into_top, BaseNote, BaseNoteSelection, EqualTemperateNoteOffset, VIBRATO_FACTOR,
};

mod notes;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let timer = dp.TC0;

    let pwm_sound_driver = dp.TC1;
    pwm_sound_driver
        .tccr1a
        .write(|w| w.com1a().match_toggle().wgm1().bits(1));
    pwm_sound_driver
        .tccr1b
        .write(|w| w.wgm1().bits(0b10).cs1().direct());

    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut reader_pin = new_console(pins.d6, timer);
    pins.d9.into_output(); // oc1a is pb1, which is d9 on arduino nano - setting high for pwm output

    led_pin.set_low();
    let mut currently_selected_note: Option<BaseNote>;
    let mut n64_controller_state = JoybusControllerState::default();
    let mut vibrato_counter: i8 = 0;
    let mut vibrato_count_direction = 1i8;
    const VIBRATO_MARGIN: i8 = 4;
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
            Ok(note) => Some(note),
            Err(0) => None,
            Err(e) => {
                let _ = uwriteln!(
                    serial,
                    "Simultaniously selected notes (counting {}), not switching.\r",
                    e
                );
                continue;
            }
        };

        let top = match currently_selected_note {
            Some(note) => {
                let power = <BaseNote as Into<EqualTemperateNoteOffset>>::into(note)
                    - n64_controller_state.z_button() as i8  // augments half step down
                    + n64_controller_state.y_axis().signum() * 2 // augments a whole step
                    + n64_controller_state.right_trigger() as i8; // augments half step up
                let freq = (power.into_frequency::<440>()
                    * if vibrato_counter == 0 {
                        1.0
                    } else {
                        let n64_modulation = n64_controller_state.x_axis().abs() as u8;
                        let n64_modulation = if n64_modulation == 0 {
                            1.0
                        } else {
                            1.0 + n64_modulation as f32 / 128.0
                        };
                        1.0 + n64_modulation * VIBRATO_FACTOR * vibrato_counter as f32
                    }) as u16;
                if vibrato_counter.abs() >= VIBRATO_MARGIN {
                    vibrato_count_direction *= -1;
                }
                vibrato_counter += vibrato_count_direction;
                frequency_into_top(freq)
            }
            None => 0,
        };
        pwm_sound_driver.ocr1a.write(|w| w.bits(top));
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
