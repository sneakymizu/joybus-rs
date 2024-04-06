#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use panic_halt as _;
use ufmt::uwriteln;
use arduino_hal::port::{PinOps, mode::{Input, Output, PullUp}, Pin};
use crate::same_pin_io::SwitchablePin;
use core::arch::asm;

mod same_pin_io;

const READ_COMMAND: u16 = 0b11;
const READ_COMMAND_LENGTH: u8 = 9;
const STATE_RESPONSE_LENGTH: u8 = 32;


fn wait_1us(){
    // these are magic... changing any instruction will result in unexpected faster execution
    unsafe{
        asm!(
            "ldi {RTMP}, 3",
		    "1:",
			"dec {RTMP}",
			"brne 1b",
            RTMP = in(reg) 4u8,
        );
    }
}
fn wait_2us(){ 
    // this does not need to be as precise. Reading in the middle of a bit should be
    // sufficient
    wait_1us();
    wait_1us();
}
fn wait_3us(){
    // these are magic... changing any instruction will result in unexpected faster execution
    unsafe{
        asm!(
            "ldi {RTMP}, 13",
		    "1:",
			"dec {RTMP}",
			"brne 1b",
            RTMP = in(reg) 15u8,
        )
    }
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
        let mut output_pin = self.0.as_output().ok_or(2u8)?;
        N64PollsignalSender::new(&mut output_pin).send()?;
        let input_pin = self.0.as_input().ok_or(2u8)?;
		Ok(N64ResponseReceiver::new(&input_pin).read()?)
	}
}

type OutputPin<PIN> = Pin<Output, PIN>;
struct N64PollsignalSender<'a, PIN>{
    pin: &'a mut OutputPin<PIN>
}
impl<'a, PIN: PinOps> N64PollsignalSender<'a, PIN>{
    pub fn new(pin: &'a mut OutputPin<PIN>)->Self{
        N64PollsignalSender{
            pin
        }
    }
    pub fn send(&mut self)->Result<(),u8>{
		for i in (0..READ_COMMAND_LENGTH).rev(){
            if 1<<i&READ_COMMAND > 0 {
                self.send_1()?;
            }
            else{
                self.send_0()?;
            }
		}
        Ok(())
    }
    fn send_0(&mut self)->Result<(),u8>{
        self.pin.set_low();
        wait_3us();
        self.pin.set_high();
        wait_1us();
        Ok(())
    }
    fn send_1(&mut self)->Result<(),u8>{
        self.pin.set_low();
        wait_1us();
        self.pin.set_high();
        wait_3us();
        Ok(())
    }
}

type InputPin<PIN> = Pin<Input<PullUp>, PIN>;
struct N64ResponseReceiver<'a, PIN>{
    pin: &'a InputPin<PIN>
}
impl<'a, PIN: PinOps> N64ResponseReceiver<'a, PIN>{
    pub fn new(pin: &'a InputPin<PIN>)->Self{
        N64ResponseReceiver{
            pin
        }
    }
    pub fn read(&mut self)->Result<u32, u8>{
        let mut res: u32 = 0;
        for bit in 0..STATE_RESPONSE_LENGTH{
            wait_2us();
            res |= (self.pin.is_high() as u32) << bit;
            wait_2us();
        }
        wait_2us();
        if self.pin.is_low(){
            Err(3u8)
        }else{
            Ok(res)
        }
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
