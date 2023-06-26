#![no_std]
#![no_main]

use core::mem::size_of;
use core::sync::atomic::{AtomicU8, Ordering};
use arduino_hal::clock::Clock;
use arduino_hal::hal::port::{PD0, PD1, PD6};
use arduino_hal::prelude::*;
use arduino_hal::port::mode::{AnyInput, Input, Output, PullUp};
use arduino_hal::port::{Pin, PinOps};
use arduino_hal::Usart;
use avr_device::atmega328p::tc0::tccr0b::CS0_A;
use avr_device::atmega328p::{TC0, USART0};
use panic_halt as _;
use ufmt::uwriteln;

static NEXT_COMPARE: AtomicU8 = AtomicU8::new(0);

struct SwitchablePin<'a, PIN: PinOps> {
    read_pin: Option<Pin<Input<PullUp>, PIN>>,
    write_pin: Option<Pin<Output, PIN>>,
    wave_gen: &'a dyn N64BitGeneration
}
impl<'a, PIN: PinOps> SwitchablePin<'a, PIN>{
    fn from_output(pin: Pin<Output, PIN>, wave_gen: &'a dyn N64BitGeneration) -> Self{
        SwitchablePin {
            read_pin: None,
            write_pin: Some(pin),
            wave_gen,
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
type SerialType = Usart<USART0, Pin<Input<AnyInput>, PD0>, Pin<Output, PD1>>;
static mut SERIAL: Option<SerialType> = None;
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    unsafe{ SERIAL = Some(arduino_hal::default_serial!(dp, pins, 57600) as SerialType)};
    dp.TC0.configure();
    let mut read_write_pin = SwitchablePin::from_output(pins.d6.into_output(), &dp.TC0);

    uwriteln!(unsafe{SERIAL.as_mut().unwrap()}, "Lets go").void_unwrap();
    led_pin.set_low();
    loop {
        //data_pin.set_high();
        //data_pin.set_low();
        //data_pin.set_high();
        //data_pin.set_low();
        //match data_pin.as_output(){
        //    Some(pin)=>{
        //        pin.send_bits(1+4+16+64);
        //        pin.set_low();
        //    },
        //    None=>()
        //};
        //let read_bits = match data_pin.as_input(){
        //    Some(pin)=>pin.read_bits(),
        //    None=>0
        //};
        //led_pin.toggle();
        //arduino_hal::delay_ms(200);
        //led_pin.toggle();
        arduino_hal::delay_ms(100);
        //uwriteln!(&mut SERIAL, "Should read {}", read_bits).void_unwrap();
    }
}
trait N64BitGeneration {
    fn configure(&self);
    fn set_high(&self);
    fn set_low(&self);
}
impl N64BitGeneration for TC0{
    fn configure(&self) {
        // setup fast pwm set on bottom clear on compare, varray high length by setting ocr0a via interrupt
        let ocr0a_value: u8 = 16; // 1MHz for 1us
        self.tccr0a.write(|w| w.com0a().bits(0b11).wgm0().bits(0b11));
        self.tccr0b.write(|w| w.wgm02().clear_bit().cs0().variant(CS0_A::PRESCALE_64));
        self.ocr0a.write(|w|w.bits(ocr0a_value));
    }
    fn set_high(&self){
        NEXT_COMPARE.store(255, Ordering::SeqCst);
    }
    fn set_low(&self){
        NEXT_COMPARE.store(0, Ordering::SeqCst);
    }
}

trait N64Communication<Bits>{
    fn send_bits(&mut self, data: Bits);
    fn recv_bits(&mut self)->u32;
}
impl<'a> N64Communication<u8> for SwitchablePin<'a, PD6>{
    fn send_bits(&mut self, data: u8){
    }
    fn recv_bits(&mut self)->u32{
        0
    }
}
