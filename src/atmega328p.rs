#[cfg(atmega328p)]

use core::arch::asm;
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
pub enum ReadError{
    OutOfMemory(u8),
    StopConditionMissmatch(u8),
    UnknownError(u8),
}

const MINIMUM_LOW_CYCLES_FOR_0:u8=40; // each loop for pin check might exit with 4 cycles wasted
const MAXIMUM_LOW_CYCLES_FOR_1:u8=17; // compares against lower
const MAXIMUM_LOW_CYCLES_FOR_CONTROLLER_STOP:u8=33; // compares against lower

#[inline]
pub fn read_bytes<const PIN: u8, const PIN_NUMBER: u8, const TIMER: u8>(data:&mut [u8;4])->Result<u8, ReadError>{
    let [high_addr, low_addr] = (data.as_ptr() as u16).to_be_bytes(); // 3c
    let mut errors:u8;
    let mut bytes_read_or_additional_error_information=0u8;
    let read_bit_position=1u8;
    unsafe{
        asm!{
            "ld {current_byte} z",
            // wait for low
            "2:",
                "sbic {pin} {pin_number}",
                "rjmp 2b",

            // there should be at least 13 cycles here to do some memory management
            "out {timer_counter_register} 0", // 21c worst case -> 11 cycles remaining until high is expected
            "cpi {read_bit_position} 0",
            "breq 0f",
            "st z+ {current_byte}",
            "inc {bytes_read}",
            "ldi {read_bit_position} 1",
            "cpi {bytes_read} 5", // ensure we're not reading beyond our memory
            "breq 99f",
            "ld {current_byte} z", // else load byte
            // end of the stuff that might be tricky to do
            // in the last high microsecond of a logic 0

            // check for high again
            "0:",
                "sbis {pin} {pin_number}",
                "rjmp 0b",
            // check time sample
            "in {low_time_register} {timer_counter_register}", // 21c worst case -> 11 cycles remaining
            "cpi {low_time_register} {low}",
            "brge 0f",
            "cpi {low_time_register} {high}",
            "brlo 1f",
            "cpi {low_time_register} {controller_stop}",
            "brlo 100f",
            "rjmp 98f",
            // store time sample
            "1:",
                "lsl {current_byte}",
                "ori {current_byte} 1",
                 // as 1 has a broader window for checking next low, the jump should be done here not for reading 0
                "rjmp 3f",
            "0:",
                "lsl {current_byte}",
            "3:",
                "lsl {read_bit_position}",
                "rjmp 2b",

            // errors and exit
            "98:",
                "ldi {errors} 2",
                "mov {bytes_read} {low_time_register}",
                "rjmp 101f",
            "99:",
                "ldi {errors} 1",
                "rjmp 101f",
            "100:",
                "ldi {errors} 0",
            "101:",
            read_bit_position=in(reg) read_bit_position,
            bytes_read=inout(reg) bytes_read_or_additional_error_information,
            errors=out(reg) errors,
            low_time_register=out(reg) _,
            current_byte=out(reg) _,
            pin=const PIN,
            pin_number=const PIN_NUMBER,
            timer_counter_register=const TIMER,
            in("ZL") low_addr,
            in("ZH") high_addr,
            low=const MINIMUM_LOW_CYCLES_FOR_0,
            high=const MAXIMUM_LOW_CYCLES_FOR_1,
            controller_stop=const MAXIMUM_LOW_CYCLES_FOR_CONTROLLER_STOP,
        }
    }
    match errors{
        0=>Ok(bytes_read_or_additional_error_information),
        1=>Err(ReadError::StopConditionMissmatch(bytes_read_or_additional_error_information)),
        2=>Err(ReadError::OutOfMemory(bytes_read_or_additional_error_information)),
        _=>Err(ReadError::UnknownError(errors)),
    }
}