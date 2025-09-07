#[cfg(atmega328p)]

use core::arch::asm;
use arduino_hal::{pac::tc0::TCNT0, port::{mode::{Input, PullUp}, Pin}};
use ufmt::derive::uDebug;

// assumes the given port is configured as output
// designed for atmega running with 16MHz clock
// 1µs is 16 clock cylces, 3µs is 48
#[inline]
pub fn send_byte<const PORT:u8 ,const PIN_NUMBER:u8>(byte: u8) {
    let bit_counter = 9u8; // 8 bits but we'll branch on zero, thus would skip the last bit.
    unsafe{
        asm!{
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
            input=in(reg) byte,
            inner_loop_counter=out(reg) _,
            port=const PORT,
            pin=const PIN_NUMBER, // should be the same for port, ddr and pmsk
        }
    }
}
#[derive(uDebug)]
pub enum ReadError{ // prolly yagni
    Timeout,
    SignalStayedLow(u8),
    SignalStayedHigh,
    StopConditionMissmatch(u8),
    UnknownError(u8),
}
impl From<u8> for ReadError{

    fn from(value: u8) -> Self {
        match value{
            TIMEOUT_ERROR=>ReadError::Timeout,
            _=>ReadError::UnknownError(value)
        }
    }
}
const TIMEOUT_ERROR: u8=1;

#[inline]
pub fn read_bytes(pin: &Pin<Input<PullUp>>, timer_counter: *mut u8, data:&mut [u8;4])->Result<u8, ReadError>{
    let mut current_bit_index = 128u8;
    let mut current_byte_index = 0u8;
    loop{
        while pin.is_high(){}
        unsafe {
            *timer_counter = 0;
        }
        while pin.is_low(){}
        let v = unsafe {
            *timer_counter
        };
        data[current_byte_index as usize] |= if v <= 16{
            current_bit_index
        }else if v <=32{
            break  // stop bit
        }else{
            0
        };
        current_bit_index=current_bit_index>>1;
        if current_bit_index==0{
            current_bit_index=128u8;
            current_byte_index+=1;
        }
        if current_byte_index>5{
            break;
        }
    }
    Ok(current_byte_index)
}