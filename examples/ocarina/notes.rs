use arduino_hal::{
    clock::Clock,
    pac::TC1,
    port::{mode::Output, Pin, PinOps},
};
use core::ops::{Add, BitAnd, Div, Mul, Sub};

use ufmt::derive::uDebug;

#[allow(dead_code)]
pub enum Sound {
    Nothing,
    Note(Note),
    Modulation(Note, i8),
}

pub struct PwmOcarina<PIN: PinOps, TIMER> {
    timer: TIMER,
    vibrato_counter: i8,
    vibrato_count_direction: i8,
    _used_output_pin: Pin<Output, PIN>,
}
impl<PIN: PinOps, TIMER> PwmOcarina<PIN, TIMER> {
    const VIBRATO_MARGIN: i8 = 4;
    const VIBRATO_FACTOR: f32 = Note::SEMITONE_FACTOR / 500.0;

    pub fn from_timer(pin: Pin<Output, PIN>, mut timer: TIMER) -> Self
    where
        TIMER: Timer,
    {
        timer.configure_timer();
        Self {
            timer,
            vibrato_counter: 0,
            vibrato_count_direction: 1i8,
            _used_output_pin: pin,
        }
    }

    pub fn play_sound<const TUNING: u32>(&mut self, sound: Sound)
    where
        TIMER: Timer,
    {
        let freq = match sound {
            Sound::Note(note) => note.into_frequency::<TUNING>() as u16,
            Sound::Modulation(note, modulation) => {
                (note.into_frequency::<TUNING>()
                    * if self.vibrato_counter == 0 {
                        1.0
                    } else {
                        let n64_modulation = if modulation == 0 {
                            1.0
                        } else {
                            1.0 + modulation as f32 / 128.0
                        };
                        1.0 + n64_modulation * Self::VIBRATO_FACTOR * self.vibrato_counter as f32
                    }) as u16
            }
            Sound::Nothing => {
                self.timer.set_timer_value(0);
                return;
            }
        };

        if self.vibrato_counter.abs() >= Self::VIBRATO_MARGIN {
            self.vibrato_count_direction *= -1;
        }
        self.vibrato_counter += self.vibrato_count_direction;
        let top = frequency_into_top(freq);
        self.timer.set_timer_value(top);
    }
}

pub trait Timer {
    fn configure_timer(&mut self);
    fn set_timer_value(&mut self, top: u16);
}
impl Timer for TC1 {
    fn configure_timer(&mut self) {
        self.tccr1a
            .write(|w| w.com1a().match_toggle().wgm1().bits(1));
        self.tccr1b.write(|w| w.wgm1().bits(0b10).cs1().direct());
    }
    fn set_timer_value(&mut self, top: u16) {
        self.ocr1a.write(|w| w.bits(top));
    }
}

/// Represents an equal temperate note.
#[derive(Clone, Copy)]
pub struct Note {
    power: i8,
}
impl Note {
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
impl Add<i8> for Note {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        Self {
            power: self.power + rhs,
        }
    }
}
impl Sub<i8> for Note {
    type Output = Self;

    fn sub(self, rhs: i8) -> Self::Output {
        Self {
            power: self.power - rhs,
        }
    }
}

#[derive(uDebug, Clone, Copy)]
pub enum BaseNote {
    D5,
    F5,
    A5,
    B5,
    D6,
}
#[derive(uDebug)]
pub struct BaseNoteSelection {
    // bitmask D,B,A,F,D
    selection: u8,
}
impl BaseNoteSelection {
    pub fn from_notes(selected_notes: u8) -> Self {
        Self {
            selection: selected_notes,
        }
    }
}
impl BitAnd<u8> for BaseNoteSelection {
    type Output = u8;

    fn bitand(self, rhs: u8) -> Self::Output {
        self.selection & rhs
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
impl From<BaseNote> for Note {
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
impl TryFrom<BaseNoteSelection> for Note {
    type Error = u8;

    fn try_from(value: BaseNoteSelection) -> Result<Self, Self::Error> {
        let base_note: BaseNote = value.try_into()?;
        Ok(base_note.into())
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
