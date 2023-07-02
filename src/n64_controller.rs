use core::mem::size_of;

use arduino_hal::port::{PinOps};
use avr_device::atmega328p::tc0::tccr0b::CS0_A;
use avr_device::atmega328p::TC0;
use crate::same_pin_io::SwitchablePin;

pub struct N64ControllerConnection<'a, PIN: PinOps, TIM> {
    connected_pin: SwitchablePin<PIN>,
    timer: &'a dyn N64BitGeneration<TIM>,
}

impl<'a, PIN: PinOps, TIM> N64ControllerConnection<'a, PIN, TIM> {
    pub fn from_pin(pin: SwitchablePin<PIN>, timer: &'a dyn N64BitGeneration<TIM>) -> Self {
        timer.configure();
        N64ControllerConnection {
            connected_pin: pin,
            timer,
        }
    }
    pub fn send_recv(&mut self, send_bits: u8) -> Result<u32, ()> {
        let mut res: u32 = 0;
        if let Some(output_pin) = self.connected_pin.as_output().as_mut() {
            for bit in 0..size_of::<u8>() * 8 {
                output_pin.set_low();
                let (high, low) = match send_bits >> bit {
                    1 => (3, 1),
                    _ => (1, 3),
                };
                self.timer.ticks(low);
                output_pin.set_high();
                self.timer.ticks(high);
            }
        } else {
            return Err(());
        }
        if let Some(input_pin) = self.connected_pin.as_input() {
            for bit in 0..size_of::<u32>() * 8 {
                res += (input_pin.is_high() as u32) << bit;
            }
        } else {
            return Err(());
        }
        Ok(res)
    }
}

pub trait N64BitGeneration<TIM> {
    fn configure(&self);
    fn ticks(&self, wait_ticks: u8);
    fn disable(&self);
    fn enable(&self);
}

impl N64BitGeneration<TC0> for TC0 {
    fn configure(&self) {
        // 1MHz for 1us
        let ocr0a_value: u8 = 16;
        // setting ctc counting us
        self.tccr0a.write(|w| w.wgm0().bits(0b10));
        self.tccr0b.write(|w| w.wgm02().clear_bit());
        self.ocr0a.write(|w| w.bits(ocr0a_value));
    }
    fn ticks(&self, wait_ticks: u8) {
        self.enable();
        for _ in 0..wait_ticks {
            while self.tifr0.read().ocf0a().bit_is_clear() {}
            self.tifr0.write(|w| w.ocf0a().set_bit());
        }
        self.disable();
    }

    fn disable(&self) {
        self.tccr0b.write(|w| w.cs0().variant(CS0_A::NO_CLOCK));
        self.tcnt0.write(|w| w.bits(0));
    }

    fn enable(&self) {
        self.tccr0b.write(|w| w.cs0().variant(CS0_A::DIRECT));
    }
}
