use arduino_hal::{
    clock::Clock,
    pac::TC1,
    port::{mode::Output, Pin, PinOps},
};
use core::ops::BitAnd;

use ufmt::derive::uDebug;

use crate::notes::{MiddleCScale, Note};

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
        TIMER: OcarinaTimer,
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
        TIMER: OcarinaTimer,
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

pub trait OcarinaTimer {
    fn configure_timer(&mut self);
    fn set_timer_value(&mut self, top: u16);
}
impl OcarinaTimer for TC1 {
    fn configure_timer(&mut self) {
        self.tccr1a
            .write(|w| w.com1a().match_toggle().wgm1().bits(1));
        self.tccr1b.write(|w| w.wgm1().bits(0b10).cs1().direct());
    }
    fn set_timer_value(&mut self, top: u16) {
        self.ocr1a.write(|w| w.bits(top));
    }
}

#[derive(uDebug, Clone, Copy)]
pub enum OcarinaNote {
    D5,
    F5,
    A5,
    B5,
    D6,
}

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
pub enum NoteSelectionError {
    TooManyNotesToPlay(u8),
    NoNoteToPlay,
}
impl TryFrom<BaseNoteSelection> for OcarinaNote {
    type Error = NoteSelectionError;
    fn try_from(value: BaseNoteSelection) -> Result<Self, Self::Error> {
        let ones = value.selection.count_ones() as u8;
        if ones > 1 {
            Err(NoteSelectionError::TooManyNotesToPlay(ones))
        } else if value.selection & (1 << OcarinaNote::A5 as u8) > 0 {
            Ok(OcarinaNote::A5)
        } else if value.selection & (1 << OcarinaNote::D5 as u8) > 0 {
            Ok(OcarinaNote::D5)
        } else if value.selection & (1 << OcarinaNote::B5 as u8) > 0 {
            Ok(OcarinaNote::B5)
        } else if value.selection & (1 << OcarinaNote::F5 as u8) > 0 {
            Ok(OcarinaNote::F5)
        } else if value.selection & (1 << OcarinaNote::D6 as u8) > 0 {
            Ok(OcarinaNote::D6)
        } else {
            Err(NoteSelectionError::NoNoteToPlay)
        }
    }
}
impl From<OcarinaNote> for Note {
    fn from(value: OcarinaNote) -> Self {
        (match value {
            OcarinaNote::A5 => Self::from(MiddleCScale::A4),
            OcarinaNote::B5 => Self::from(MiddleCScale::B4),
            OcarinaNote::D6 => Self::from(MiddleCScale::D4) + 12,
            OcarinaNote::D5 => Self::from(MiddleCScale::D4),
            OcarinaNote::F5 => Self::from(MiddleCScale::F4),
        }) + 12
    }
}
impl TryFrom<BaseNoteSelection> for Note {
    type Error = NoteSelectionError;

    fn try_from(value: BaseNoteSelection) -> Result<Self, Self::Error> {
        let base_note: OcarinaNote = value.try_into()?;
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
