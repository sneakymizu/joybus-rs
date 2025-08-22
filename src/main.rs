#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use panic_halt as _;
use ufmt::{derive::uDebug, uDebug, uDisplay};

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
                    // no need to count here anymore
                bit_counter=in(reg) bit_counter,
                input=in(reg) input,
                inner_loop_counter=out(reg) _,
                port=const 0x0b,
                ddr=const 0x0a,
                pin=const PIN_NUMBER, // should be the same for port, ddr and pmsk
            }
        }
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
    let _ = poll_controller_state::<6>();
    loop {
        let _ = poll_controller_state::<6>();
        /*let _write = match poll_controller_state::<6>() {
            Ok(num) => uwriteln!(serial, "Have read {}\r", num.0),
            Err(e) => uwriteln!(serial, "Error occured {}\r", e),
        };*/
        arduino_hal::delay_ms(100);
    }
}
