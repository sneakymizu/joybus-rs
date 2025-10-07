#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use core::ops::{Add, BitAnd, Div, Mul, Sub};

use arduino_hal::clock::Clock;
use panic_halt as _;
use ufmt::{derive::uDebug, uwriteln};

use joybus_rs_atmega328p::new_console;

use joybus_rs::{JoybusConsole, JoybusConsoleExt, JoybusControllerState, JoybusError};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let timer = dp.TC0;
    // normal operating timer
    timer.tccr0a.reset();
    timer.tccr0b.write(|w| w.cs0().direct()); // no prescale, normal timer operation
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
    let mut reader_pin = new_console(pins.d6);
    pins.d9.into_output(); // oc1a is pb1, which is d9 on arduino nano - setting high for pwm output

    led_pin.set_low();
    let mut currently_selected_note: Option<BaseNote>;
    let mut n64_controller_state = JoybusControllerState::default();
    let mut vibrato_counter: i8 = 0;
    let mut vibrato_count_direction = 1i8;
    const VIBRATO_MARGIN: i8 = 4;
    loop {
         match reader_pin.read_contoller_state(&mut n64_controller_state) {
            Ok(_) => (),
            Err(JoybusError::OutOfMemory(len)) => {
                let _ = uwriteln!(serial, "(No Stopbit) Bytes are {:?}: {:?}\r", len, n64_controller_state);
                continue;
            }
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

const VIBRATO_FACTOR: f32 = EqualTemperateNoteOffset::SEMITONE_FACTOR / 500.0;

#[derive(Clone, Copy)]
struct EqualTemperateNoteOffset {
    power: i8,
}
impl EqualTemperateNoteOffset {
    const SEMITONE_FACTOR: f32 = 1.05946309436; // 1/12

    fn into_frequency<const BASE_FREQUENCY: u32>(self) -> f32 {
        let mut frequency = BASE_FREQUENCY as f32;
        let fun = if self.power >= 0 {
            <f32 as Mul>::mul
        } else {
            <f32 as Div>::div
        };
        for _ in 0..self.power.abs() {
            frequency = fun(frequency, Self::SEMITONE_FACTOR);
        }
        frequency + 0.5
    }
}
impl Add<i8> for EqualTemperateNoteOffset {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        Self {
            power: self.power + rhs,
        }
    }
}
impl Sub<i8> for EqualTemperateNoteOffset {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        Self {
            power: self.power - rhs,
        }
    }
}

#[derive(uDebug, Clone, Copy)]
enum BaseNote {
    D5,
    F5,
    A5,
    B5,
    D6,
}
#[derive(uDebug)]
struct BaseNoteSelection {
    // bitmask D,B,A,F,D
    selection: u8,
}
impl BitAnd<u8> for BaseNoteSelection {
    type Output = u8;

    fn bitand(self, rhs: u8) -> Self::Output {
        self.selection & rhs
    }
}
impl From<&JoybusControllerState> for BaseNoteSelection {
    fn from(value: &JoybusControllerState) -> Self {
        let note_selections = (value.c_right() as u8) << BaseNote::A5 as u8
            | (value.c_left() as u8) << BaseNote::B5 as u8
            | (value.a_button() as u8) << BaseNote::D5 as u8
            | (value.c_down() as u8) << BaseNote::F5 as u8
            | (value.c_up() as u8) << BaseNote::D6 as u8;
        Self {
            selection: note_selections,
        }
    }
}
impl TryFrom<BaseNoteSelection> for BaseNote {
    type Error = u8;
    fn try_from(value: BaseNoteSelection) -> Result<Self, Self::Error> {
        let ones = value.selection.count_ones() as u8;
        if ones > 1 {
            Err(ones)
        } else if value.selection & (1 << BaseNote::A5 as u8) > 0 {
            Ok(BaseNote::A5)
        } else if value.selection & (1 << BaseNote::D5 as u8) > 0 {
            Ok(BaseNote::D5)
        } else if value.selection & (1 << BaseNote::B5 as u8) > 0 {
            Ok(BaseNote::B5)
        } else if value.selection & (1 << BaseNote::F5 as u8) > 0 {
            Ok(BaseNote::F5)
        } else if value.selection & (1 << BaseNote::D6 as u8) > 0 {
            Ok(BaseNote::D6)
        } else {
            Err(0)
        }
    }
}
impl From<BaseNote> for EqualTemperateNoteOffset {
    fn from(value: BaseNote) -> Self {
        let note = match value {
            BaseNote::A5 => Self { power: 0 },
            BaseNote::B5 => Self { power: 2 },
            BaseNote::D6 => Self { power: 5 },
            BaseNote::D5 => Self { power: -7 },
            BaseNote::F5 => Self { power: -4 },
        };
        note + 12
    }
}

fn frequency_into_top(freq: u16) -> u16 {
    // only works for 16 bit phase and frequency correct timer
    if freq == 0 {
        0
    } else {
        (arduino_hal::DefaultClock::FREQ / (4 * freq as u32)) as u16
    }
}
