#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use panic_halt as _;
use ufmt::{derive::uDebug, uDebug, uDisplay, uwriteln};

pub struct N64ControllerState(u32);
#[derive(uDebug)]
pub enum ReadError{
    Failed
}
impl uDisplay for ReadError{
    fn fmt<W>(&self, fmt: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized {
        uDebug::fmt(&self, fmt)
    }
}
pub trait PollN64{
    fn read(&self)->Result<N64ControllerState, ReadError>;
}

#[cfg(atmega328p)]
mod atmega328p_read{
    use core::arch::asm;
    use super::{PollN64, N64ControllerState, ReadError};

    pub struct Reader;
    impl PollN64 for Reader{
        fn read(&self)->Result<N64ControllerState, ReadError>{
            unsafe{
                let zeros:u8 = 7;
                let ones:u8 = 2;
                asm!{
                    //poll
                    "sbi {port}, {pin}", // PORTB Pin "6"
                    "sbi {ddr}, {pin}", // DDRB Pin "6"
                    "0:",
                        "cbi {port}, {pin}",
                        "ldi {rtmp}, 15",
                        "10:",
                            "dec {rtmp}",
                            "brne 10b",
                        "nop",
                        "sbi {port}, {pin}",
                        "ldi {rtmp}, 3",
                        "11:",
                            "dec {rtmp}",
                            "brne 11b",
                        "nop",
                        "nop",
                        "dec {zeros}",
                        "brne 0b",
                    "1:",
                        "cbi {port}, {pin}",
                        "ldi {rtmp}, 4",
                        "10:",
                            "dec {rtmp}",
                            "brne 10b",
                        "nop",
                        "nop",
                        "sbi {port}, {pin}",
                        "ldi {rtmp}, 14",
                        "11:",
                            "dec {rtmp}",
                            "brne 11b",
                        "nop",
                        "dec {ones}",
                        "brne 1b",
                    "nop",
                    //switch to read
                    "sbi {port}, {pin}",
                    "cbi {ddr}, {pin}",
                    zeros=in(reg) zeros,
                    ones=in(reg) ones,
                    rtmp=out(reg)_,
                    port=const 0x0b,
                    ddr=const 0x0a,
                    pin=const 0x06
                }
            }
            Ok(N64ControllerState(1))
        }
    }
}
#[cfg(atmega328p)]
pub use atmega328p_read::Reader;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let _reader_pin = pins.d6.into_output_high();
    let n64 = Reader;

    uwriteln!(serial, "Lets go\r").unwrap();
    led_pin.set_low();
    loop {
        let _write = match n64.read() {
            Ok(num) => uwriteln!(serial, "Have read {}\r", num.0),
            Err(e) => uwriteln!(serial, "Error occured {}\r", e),
        };
        arduino_hal::delay_ms(1000);
    }
}
