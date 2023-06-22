#![no_std]
#![no_main]

use core::mem::size_of;
use arduino_hal::prelude::*;
use arduino_hal::port::mode::{Input, Output, PullUp};
use arduino_hal::port::{Pin, PinOps};
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

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    let mut data_pin = SwitchablePin::from_output(pins.a0.into_output());
    led_pin.set_high();
    loop {
        match data_pin.as_output(){
            Some(pin)=>pin.send_bits(3),
            None=>()
        };
        let read_bits = match data_pin.as_input(){
            Some(pin)=>pin.read_bits(),
            None=>0
        };
        led_pin.toggle();
        arduino_hal::delay_ms(200);
        led_pin.toggle();
        arduino_hal::delay_ms(200);
        uwriteln!(&mut serial, "Should read {}", read_bits).void_unwrap();
    }
}

trait SendN64Bits{
    fn send_bits(&mut self, data: usize);
}
trait RecvN64Bits{
    fn read_bits(&self)->u32;
}

impl<PIN: PinOps> SendN64Bits for Pin<Output, PIN>{
    fn send_bits(&mut self, data: usize) {
        for count in 0..=size_of::<usize>()*8{
            if data & 1<<count != 0{
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
