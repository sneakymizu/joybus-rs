#![no_std]
#![no_main]

use core::mem::size_of;
use arduino_hal::clock::Clock;
use arduino_hal::hal::Atmega;
use arduino_hal::hal::port::{PD0, PD1};
use arduino_hal::prelude::*;
use arduino_hal::port::mode::{AnyInput, Input, Output, PullUp};
use arduino_hal::port::{Pin, PinOps};
use arduino_hal::Usart;
use avr_device::atmega328p::tc0::tccr0b::CS0_A;
use avr_device::atmega328p::{TC0, USART0};
use panic_halt as _;
use ufmt::uwriteln;

struct SwitchablePin<PIN: PinOps>{
    read_pin: Option<Pin<Input<PullUp>, PIN>>,
    write_pin: Option<Pin<Output, PIN>>
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
type SerialType = Usart<USART0, Pin<Input<AnyInput>, PD0>, Pin<Output, PD1>>;
static mut SERIAL: Option<SerialType> = None;
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    unsafe{ SERIAL = Some(arduino_hal::default_serial!(dp, pins, 57600) as SerialType)};
    uwriteln!(unsafe{SERIAL.as_mut().unwrap()}, "Lets go").void_unwrap();
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    let mut data_pin = pins.a0.into_output();
    data_pin.set_low();
    let mut tmr = dp.TC0;
    configure_timer(&mut tmr);
    led_pin.set_high();
    loop {
        let val = unsafe{tmr.tifr0.read().tov0()};
        if val.bit_is_set(){
            data_pin.toggle();
        }
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
fn configure_timer(tmr: &mut TC0){
    const freq: u32 = arduino_hal::DefaultClock::FREQ;
    const CLOCK_SOURCE: CS0_A = CS0_A::PRESCALE_64;
    const tcnt0_value: u8 = 255;
    tmr.tccr0a.write(|w| w.wgm0().bits(0b00));
    tmr.tccr0b.write(|w| w.wgm02().clear_bit().cs0().variant(CLOCK_SOURCE));
    tmr.tcnt0.write(|w| w.bits(tcnt0_value))
    //tmr.timsk0.write(|w| w.ocie0a().set_bit());
}

trait SendN64Bits<B>{
    fn send_bits(&mut self, data: B);
}
trait RecvN64Bits{
    fn read_bits(&self)->u32;
}

impl<PIN: PinOps> SendN64Bits<u8> for Pin<Output, PIN>{
    fn send_bits(&mut self, data: u8) {
        //let se =unsafe{SERIAL.as_mut().unwrap()};
        let end_of_loop = size_of::<u8>()*8;
        //uwriteln!(se, "Start loop to {}", end_of_loop).void_unwrap();
        for count in 0..=end_of_loop{
            let val = (data >> count & 1) != 0;
            //uwriteln!(se, "Should send {} as {} bit", val, count).void_unwrap();
            if val {
                self.set_high();
            }else{
                self.set_low();
            }
        }
    }
}
impl<PIN: PinOps> RecvN64Bits for Pin<Input<PullUp>, PIN>{
    fn read_bits(&self) -> u32 {
        self.is_high();
        self.is_low();
        0
    }
}
