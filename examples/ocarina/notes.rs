use arduino_hal::clock::Clock;
use core::ops::{Add, BitAnd, Div, Mul, Sub};

use ufmt::derive::uDebug;
pub const VIBRATO_FACTOR: f32 = EqualTemperateNoteOffset::SEMITONE_FACTOR / 500.0;

pub struct PwmOcarina {}

#[derive(Clone, Copy)]
pub struct EqualTemperateNoteOffset {
    power: i8,
}
impl EqualTemperateNoteOffset {
    const SEMITONE_FACTOR: f32 = 1.05946309436; // 1/12

    pub fn into_frequency<const BASE_FREQUENCY: u32>(self) -> f32 {
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

pub fn frequency_into_top(freq: u16) -> u16 {
    // only works for 16 bit phase and frequency correct timer
    if freq == 0 {
        0
    } else {
        (arduino_hal::DefaultClock::FREQ / (4 * freq as u32)) as u16
    }
}
