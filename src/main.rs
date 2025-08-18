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
            let read_loop_counter: u8 = 4;
            let reg0: u8;
            let reg1: u8;
            let reg2: u8;
            let reg3: u8;
            let zeros:u8 = 7;
            let ones:u8 = 2;
            unsafe{
                asm!{
                    //poll
                    "sbi {port}, {pin}", // PORTD Pin "6"
                    "sbi {ddr}, {pin}", // DDRD Pin "6"
                    "0:",
                        "cbi {port}, {pin}",
                        "ldi {inner_loop_counter}, 15",
                        "10:",
                            "dec {inner_loop_counter}",
                            "brne 10b",
                        "nop",
                        "sbi {port}, {pin}",
                        "ldi {inner_loop_counter}, 3",
                        "11:",
                            "dec {inner_loop_counter}",
                            "brne 11b",
                        "nop",
                        "nop",
                        "dec {zeros}",
                        "brne 0b",
                    "1:",
                        "cbi {port}, {pin}", // 2
                        "ldi {inner_loop_counter}, 4", // 1
                        "10:",
                            "dec {inner_loop_counter}", // 1
                            "brne 10b", // 1/2
                        "nop",
                        "nop",
                        "sbi {port}, {pin}",
                        "ldi {inner_loop_counter}, 14",
                        "11:",
                            "dec {inner_loop_counter}",
                            "brne 11b",
                        "nop",
                        "dec {ones}",
                        "brne 1b",
                    "nop", // 1
                    //switch to read
                    "sbi {port}, {pin}", // 1
                    "cbi {ddr}, {pin}", // 1
                    "ldi {read_loop_counter}, 4", // 1
                    "0:",
                        // wait 1µs
                        "ldi {inner_loop_counter}, 4", // 1
                        "10:",
                            "dec {inner_loop_counter}", // 1
                            "brne 10b", // 1/2
                        "dec {read_loop_counter}",
                        // 15 cycles 16 is 1µs
                        "sbrs {port}, {pin}", // 1 if not set 2 if set
                        "jmp 0b", // 3 which is equiv to sbi+cbi+lbi so we should get to sbrs with 15 cylces again
                        // sbrs takes 2 cycles so "high" starts with 1
                        "cpi {read_loop_counter}, 3", // 1
                        "brlt 0f", // 1/2
                        "jmp 1f",
                        "0:",
                            "add {reg0}, 1",
                    "1:",
                        // wait 1µs
                        "ldi {inner_loop_counter}, 4", // 1
                        "10:",
                            "dec {inner_loop_counter}", // 1
                            "brne 10b", // 1/2
                        "dec {read_loop_counter}",
                        // 15 cycles 16 is 1µs
                        "sbrs {port}, {pin}", // 1 if not set 2 if set
                        "jmp 0b", // 3 which is equiv to sbi+cbi+lbi so we should get to sbrs with 15 cylces again
                        // sbrs takes 2 cycles so "high" starts with 1
                        "cpi {read_loop_counter}, 3", // 1
                        "brlt 0f", // 1/2
                        "jmp 2f",
                        "0:",
                            "add {reg1}, 1",
                    "2:",
                        // wait 1µs
                        "ldi {inner_loop_counter}, 4", // 1
                        "10:",
                            "dec {inner_loop_counter}", // 1
                            "brne 10b", // 1/2
                        "dec {read_loop_counter}",
                        // 15 cycles 16 is 1µs
                        "sbrs {port}, {pin}", // 1 if not set 2 if set
                        "jmp 0b", // 3 which is equiv to sbi+cbi+lbi so we should get to sbrs with 15 cylces again
                        // sbrs takes 2 cycles so "high" starts with 1
                        "cpi {read_loop_counter}, 3", // 1
                        "brlt 0f", // 1/2
                        "jmp 3f",
                        "0:",
                            "add {reg2}, 1",
                    "3:",
                        // wait 1µs
                        "ldi {inner_loop_counter}, 4", // 1
                        "10:",
                            "dec {inner_loop_counter}", // 1
                            "brne 10b", // 1/2
                        "dec {read_loop_counter}",
                        // 15 cycles 16 is 1µs
                        "sbrs {port}, {pin}", // 1 if not set 2 if set
                        "jmp 0b", // 3 which is equiv to sbi+cbi+lbi so we should get to sbrs with 15 cylces again
                        // sbrs takes 2 cycles so "high" starts with 1
                        "cpi {read_loop_counter}, 3", // 1
                        "0:",
                            "add {reg3}, 1",
                    zeros=in(reg) zeros,
                    ones=in(reg) ones,
                    inner_loop_counter=out(reg) _,
                    read_loop_counter=in(reg) read_loop_counter,
                    port=const 0x0b,
                    ddr=const 0x0a,
                    pin=const 0x06, // should be the same for port, ddr and pmsk
                    reg0=out(reg) reg0,
                    reg1=out(reg) reg1,
                    reg2=out(reg) reg2,
                    reg3=out(reg) reg3,
                }
            }
            let state = u32::from_be_bytes([reg0,reg1,reg2,reg3]);
            Ok(N64ControllerState(state))
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
