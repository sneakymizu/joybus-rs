#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use avr_device::interrupt::Mutex;
use arduino_hal::prelude::*;
use arduino_hal::port::mode::{Input, Output, PullUp};
use arduino_hal::port::{Pin, PinOps};
use panic_halt as _;
use ufmt::{uwriteln};


fn send_poll_signal<PIN: PinOps>(pin: &mut Pin<Output, PIN>){
    pin.set_high();
}
fn read_poll_data<PIN: PinOps>(pin: &Pin<Input<PullUp>, PIN>){
    pin.is_high();
    pin.is_low();
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    let mut data_pin = pins.a0.into_output();
    led_pin.set_high();
    loop {
        data_pin.send_bits(7);
        //data_pin.take().into_output().send_bits(8);
        //data_pin.as_ptr().into_pull_up_input().read_bits();
        led_pin.toggle();
        arduino_hal::delay_ms(200);
        led_pin.toggle();
        arduino_hal::delay_ms(200);
        uwriteln!(&mut serial, "Should read {}", 1).void_unwrap();
    }
}

trait SendN64Bits{
    fn send_bits(&mut self, data: u8);
}
trait RecvN64Bits{
    fn read_bits(&self, )->u8;
}

impl<PIN: PinOps> SendN64Bits for Pin<Output, PIN>{
    fn send_bits(&mut self, data: u8) {
        todo!()
    }
}
impl<PIN: PinOps> RecvN64Bits for Pin<Input<PullUp>, PIN>{
    fn read_bits(&self, ) -> u8 {
        todo!()
    }
}