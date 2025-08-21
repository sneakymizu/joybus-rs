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

#[cfg(atmega328p)]
mod atmega328p_read{
    use core::arch::asm;
    use super::{N64ControllerState, ReadError};

    pub fn poll_controller_state<const PIN_NUMBER:u8>() -> Result<N64ControllerState, ReadError>{
        let read_loop_counter: u8 = 4;
        let reg0: u8;
        let reg1: u8;
        let reg2: u8;
        let reg3: u8;
        let zeros = 7u8;
        let ones  = 2u8;
        let input = 0b10101010u8;
        let bit_counter = 9u8; // 8 bits but we'll branch on zero, thus would skip the last bit.
        unsafe{
            asm!{
                // designed for atmega running with 16MHz clock
                // 1µs is 16 clock cylces, 3µs is 48
                // send byte
                "sbi {ddr}, {pin}", // DDR Pin "PIN_NUMBER"
                "sbi {port}, {pin}", // PORT Pin "PIN_NUMBER"
                "0:",
                    "dec {bit_counter}", // 1c
                    "breq 2f", // 1c for non branching (sending 1 or 0) else done writing
                    "lsl {input}", // 1c
                    "brcs 11f", // 1c for send 0, 2c for send 1
                    "nop", //1c add nop here so one and zero cycle cout are equivalent
                // start sending logic zero here (already high for 5c)
                    "cbi {port}, {pin}", // 2c
                    "ldi {inner_loop_counter}, 15", // 1c
                    "1:",
                        "dec {inner_loop_counter}", // 1c
                        "brne 1b",  // 2c on branch else 1c
                        // exit with 45c low
                    "nop", // 1c
                    "sbi {port}, {pin}", // 2c # high on cycle 48
                    "ldi {inner_loop_counter}, 2", // 1c
                    "1:",
                        "dec {inner_loop_counter}", // 1c
                        "brne 1b", // 2c on branch else 1c
                        "nop", // 1c
                        // exit with 8 cycles
                        "breq 0b", // 2c (otherwise brne would have hit), exit with 9 cycles high here
                "11:",  // start sending logic one here (already high for 5c)
                    "cbi {port}, {pin}", // 2c
                    "ldi {inner_loop_counter}, 4", // 1c
                    "1:",
                        "dec {inner_loop_counter}", // 1c
                        "brne 1b", // 1c/2c
                    "nop", // 1c
                    "nop", // 1c
                    "sbi {port}, {pin}", // 2c - high on cycle 16
                    "ldi {inner_loop_counter}, 13", // 1c
                    "1:",
                        "dec {inner_loop_counter}", // 1c
                        "brne 1b", // 1c
                        "breq 0b", // 2c
                "2:", // send stop bit (starts with 12c/44c high)
                    "nop", // 1c
                    "nop", // 1c
                    "cbi {port}, {pin}", // 2c
                    "ldi {inner_loop_counter}, 4", // 1c
                    "1:",
                        "dec {inner_loop_counter}", // 1c
                        "brne 1b", // 1c/2c
                    "nop", // 1c
                    "nop", // 1c
                    "sbi {port}, {pin}", // 2c - high on cycle 16
                    "ldi {inner_loop_counter}, 13", // 1c
                    "1:",
                        "dec {inner_loop_counter}", // 1c
                        "brne 1b", // 1c
                // continue with reading here
                /*
                //switch to read
                "cbi {ddr}, {pin}", // 1
                "sbi {port}, {pin}", // 1
                "ldi {read_loop_counter}, 4", // 1
                "0:",
                    // wait 1µs
                    "ldi {inner_loop_counter}, 4", // 1
                    "00:",
                        "dec {inner_loop_counter}", // 1
                        "brne 00b", // 1/2
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
                        "add {reg3}, 1",*/
                bit_counter=in(reg) bit_counter,
                input=in(reg) input,
                inner_loop_counter=out(reg) _,
                //read_loop_counter=in(reg) read_loop_counter,
                port=const 0x0b,
                ddr=const 0x0a,
                pin=const PIN_NUMBER, // should be the same for port, ddr and pmsk
                //reg0=out(reg) reg0,
                //reg1=out(reg) reg1,
                //reg2=out(reg) reg2,
                //reg3=out(reg) reg3,
            }
        }
        //let state = u32::from_be_bytes([reg0,reg1,reg2,reg3]);
        let state = 1;
        Ok(N64ControllerState(state))
    }
}
#[cfg(atmega328p)]
pub use atmega328p_read::poll_controller_state;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    //let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut _reader_pin = pins.d6.into_output_high();
    _reader_pin.set_high();

    //uwriteln!(serial, "Lets go\r").unwrap();
    led_pin.set_low();
    arduino_hal::delay_ms(5000);
    loop {
        let _ = poll_controller_state::<6>();
        /*let _write = match poll_controller_state::<6>() {
            Ok(num) => uwriteln!(serial, "Have read {}\r", num.0),
            Err(e) => uwriteln!(serial, "Error occured {}\r", e),
        };*/
        arduino_hal::delay_ms(100);
    }
}
