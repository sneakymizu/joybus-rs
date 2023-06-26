#![no_std]
#![no_main]

use arduino_hal::port::Pin;
use arduino_hal::hal::port::{PD0, PD1};
use arduino_hal::port::mode::{AnyInput, Input, Output};
use arduino_hal::prelude::*;
use arduino_hal::Usart;
use avr_device::atmega328p::USART0;
use panic_halt as _;
use ufmt::uwriteln;

mod n64_controller;

use n64_controller::N64ControllerConnection;

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
    let mut read_write_pin = N64ControllerConnection::from_pin(pins.d6.into_output(), &dp.TC0);

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
