use arduino_hal::port::{Pin, PinOps};
use arduino_hal::port::mode::{Input, Output, PullUp};
use avr_device::atmega328p::TC0;
use avr_device::atmega328p::tc0::tccr0b::CS0_A;
use core::sync::atomic::{AtomicU8, Ordering};

static NEXT_COMPARE: AtomicU8 = AtomicU8::new(0);

pub struct N64ControllerConnection<'a, PIN: PinOps>{
    connected_pin: SwitchablePin<PIN>,
    timer: &'a dyn N64BitGeneration,
}
impl<'a, PIN: PinOps> N64ControllerConnection<'a, PIN>{
    pub fn from_pin(pin: Pin<Output, PIN>, timer: &'a dyn N64BitGeneration) -> Self{
        timer.configure();
        N64ControllerConnection { 
            connected_pin: SwitchablePin::from_output(pin),
            timer
        }
    }

}

struct SwitchablePin<PIN: PinOps> {
    read_pin: Option<Pin<Input<PullUp>, PIN>>,
    write_pin: Option<Pin<Output, PIN>>,
}
impl<PIN: PinOps> SwitchablePin<PIN>{
    fn from_output(pin: Pin<Output, PIN>) -> Self{
        SwitchablePin {
            read_pin: None,
            write_pin: Some(pin),
        }
    }
    fn as_output(&mut self)->Option<&mut Pin<Output, PIN>>{
        if self.read_pin.is_some(){
            self.write_pin = Some(self.read_pin.take().unwrap().into_output())
        }
        self.write_pin.as_mut()
    }
    fn as_input(&mut self)->Option<&mut Pin<Input<PullUp>, PIN>>{
        if self.write_pin.is_some(){
            self.read_pin = Some(self.write_pin.take().unwrap().into_pull_up_input());
        }
        self.read_pin.as_mut()
    }
}

pub trait N64BitGeneration {
    fn configure(&self);
    fn set_high(&self);
    fn set_low(&self);
}
impl N64BitGeneration for TC0{
    fn configure(&self) {
        // setup fast pwm set on bottom clear on compare, varray high length by setting ocr0a via interrupt
        let ocr0a_value: u8 = 16; // 1MHz for 1us
        self.tccr0a.write(|w| w.com0a().bits(0b11).wgm0().bits(0b11));
        self.tccr0b.write(|w| w.wgm02().clear_bit().cs0().variant(CS0_A::DIRECT));
        self.ocr0a.write(|w|w.bits(ocr0a_value));
    }
    fn set_high(&self){
        NEXT_COMPARE.store(255, Ordering::SeqCst);
    }
    fn set_low(&self){
        NEXT_COMPARE.store(0, Ordering::SeqCst);
    }
}

