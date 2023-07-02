use core::mem::size_of;
use arduino_hal::{delay_us};

use arduino_hal::port::{PinOps};
use crate::same_pin_io::SwitchablePin;

pub struct N64ControllerConnection<PIN: PinOps> {
    connected_pin: SwitchablePin<PIN>,
}

impl<PIN: PinOps> N64ControllerConnection<PIN> {
    pub fn from_pin(pin: SwitchablePin<PIN>) -> Self {
        N64ControllerConnection {
            connected_pin: pin,
        }
    }
    pub fn send_recv(&mut self, send_bits: u8) -> Result<u32, ()> {
        let mut res: u32 = 0;
        if let Some(output_pin) = self.connected_pin.as_output() {
            for bit in 0..size_of::<u8>() * 8 {
                output_pin.set_low();
                let (high, low) = match send_bits >> bit {
                    1 => (3, 1),
                    _ => (1, 3),
                };
                delay_us(low);
                output_pin.set_high();
                delay_us(high);
            }
        } else {
            return Err(());
        }
        if let Some(input_pin) = self.connected_pin.as_input() {
            for bit in 0..size_of::<u32>() * 8 {
                delay_us(2);
                res += (input_pin.is_high() as u32) << bit;
            }
        } else {
            return Err(());
        }
        Ok(res)
    }
}
