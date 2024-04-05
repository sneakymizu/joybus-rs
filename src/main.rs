#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use panic_halt as _;
use ufmt::uwriteln;
use arduino_hal::port::{Pin, PinOps, mode::Output};
use crate::same_pin_io::SwitchablePin;
use core::arch::asm;

mod same_pin_io;

const READ_COMMAND: u16 = 0b11;
const READ_COMMAND_LENGTH: u8 = 9;

fn wait_1us(){
    // these are magic... changing any instruction will result in unexpected faster execution
    unsafe{
        asm!(
            "ldi {RTMP}, 3",
		    "1:",
			"dec {RTMP}",
			"brne 1b",
            "nop",
            "nop",
            RTMP = in(reg) 4u8,
        );
    }
}
fn wait_3us(){
    // these are magic... changing any instruction will result in unexpected faster execution
    unsafe{
        asm!(
            "ldi {RTMP}, 14",
		    "1:",
			"dec {RTMP}",
			"brne 1b",
            RTMP = in(reg) 15u8,
        )
    }
}
fn send_0<PIN: PinOps>(pin: &mut Pin<Output, PIN>){
    pin.set_low();
    wait_3us();
    pin.set_high();
    wait_1us();
}
fn send_1<PIN: PinOps>(pin: &mut Pin<Output, PIN>){
    pin.set_low();
    wait_1us();
    pin.set_high();
    wait_3us();
}
struct N64Communicator<PIN: PinOps>(SwitchablePin<PIN>);
impl<PIN: PinOps> N64Communicator<PIN>{
    fn new(mut pin: SwitchablePin<PIN>)->Self{
        if let Some(pin) = pin.as_output(){
            pin.set_high();
        }
        Self(pin)
    }
	fn read(&mut self)->Result<u32,u8>{
		let pin = self.0.as_output().ok_or(1u8)?;
		for i in (0..READ_COMMAND_LENGTH).rev(){
            if 1<<i&READ_COMMAND > 0 {
                send_1(pin);
            }
            else{
                send_0(pin);
            }
		}
        let _pin = self.0.as_input().ok_or(2u8)?;
        arduino_hal::delay_ms(100);
        let _pin = self.0.as_output().ok_or(2u8)?;
		Ok(0)
	}
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let pin = SwitchablePin::from_output(pins.d6.into_output());
    let mut n64 = N64Communicator::new(pin);

    uwriteln!(serial, "Lets go\r").unwrap();
    led_pin.set_low();
    loop {
		let _write = match n64.read(){
			Ok(num)=>uwriteln!(serial, "Have read {}\r", num),
			Err(e)=>uwriteln!(serial, "Error occured {}\r", e),
		};
        arduino_hal::delay_ms(1);
    }
}
