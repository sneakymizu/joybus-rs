use core::cell::RefCell;
use core::mem;
use core::mem::size_of;
use arduino_hal::{delay_us};

use arduino_hal::port::{PinOps};
use avr_device::atmega328p::TC0;
use avr_device::interrupt::Mutex;
use embedded_hal::digital::{InputPin, OutputPin};
use crate::same_pin_io::SwitchablePin;

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA(){
    let pin = unsafe {
        PIN.as_mut_ptr().get_mut().get_mut()
    };
    match pin{
        N64DataDirection::Read(reader) => reader.read_next(),
        N64DataDirection::Write(poller) => poller.toggle_next()
    }
}
static mut PIN: mem::MaybeUninit<Mutex<RefCell<N64DataDirection>>> = mem::MaybeUninit::uninit();

enum N64DataDirection {
    Read(dyn N64InputReader),
    Write(dyn N64InputPoller)
}
trait N64InputReader{
    fn read_next(&self);
    fn get_value(&self) -> u32;
}
trait N64InputPoller{
    fn toggle_next(&self);
}

pub struct N64ControllerConnection<PIN: PinOps> {
    connected_pin: SwitchablePin<PIN>,
}

impl<PIN: PinOps> N64ControllerConnection<PIN> {
    pub fn from_pin(pin: SwitchablePin<PIN>, tc0: TC0) -> Self {
        configure(tc0);
        N64ControllerConnection {
            connected_pin: pin,
        }
    }
    fn configure(tc0: TC0){
        // 1MHz for 1us
        let ocr0a_value: u8 = 16;
        // setting ctc counting us
        tc0.tccr0a.write(|w| w.wgm0().bits(0b10));
        tc0.tccr0b.write(|w| w.wgm02().clear_bit());
        tc0.ocr0a.write(|w| w.bits(ocr0a_value));
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
