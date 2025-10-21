use core::ops::{Add, Div, Mul, Sub};

/// Represents an equal temperate note.
#[derive(Clone, Copy)]
pub struct Note {
    power: i8,
}
impl Note {
    pub const SEMITONE_FACTOR: f32 = 1.05946309436; // 1/12

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

#[allow(dead_code)]
pub enum Step {
    FullStep = 2,
    HalfStep = 1,
    None = 0,
}
impl Add<Step> for Note {
    type Output = Self;

    fn add(self, rhs: Step) -> Self::Output {
        self + rhs as i8
    }
}
impl Sub<Step> for Note {
    type Output = Self;

    fn sub(self, rhs: Step) -> Self::Output {
        self - rhs as i8
    }
}
impl Mul<Step> for i8 {
    type Output = i8;

    fn mul(self, rhs: Step) -> Self::Output {
        self * rhs as i8
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum MiddleCScale {
    C4 = -9,
    D4 = -7,
    E4 = -5,
    F4 = -4,
    G4 = -2,
    A4 = 0,
    B4 = 2,
}
impl From<&MiddleCScale> for Note {
    fn from(value: &MiddleCScale) -> Self {
        Note {
            power: *value as i8,
        }
    }
}
impl From<MiddleCScale> for Note {
    fn from(value: MiddleCScale) -> Self {
        Note { power: value as i8 }
    }
}
