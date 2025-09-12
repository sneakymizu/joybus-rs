#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use core::arch::asm;
use ufmt::derive::uDebug;

// assumes the given port is configured as output
// designed for atmega running with 16MHz clock
// 1µs is 16 clock cylces, 3µs is 48
const BIT_COUNTER_INIT:u8=9u8; // 8 bits but we'll branch on zero, thus would skip the last bit.
#[inline]
pub unsafe fn send_byte<const PORT:u8 ,const PIN_NUMBER:u8, const BYTES:usize>(bytes: [u8;BYTES]) {
    let bit_counter = BYTES*8+1;
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
#[derive(uDebug)]
pub enum ReadError{
    OutOfMemory(u8),
    UnknownError(u8),
}

 // each loop for pin check might exit with 4~5 cycles wasted & third party controllers aren't too specific about timing so this expects more than "stopbit"-low
const MINIMUM_LOW_CYCLES_FOR_0:u8=33;
 // 16+5 cycles, compares against lower
const MAXIMUM_LOW_CYCLES_FOR_1:u8=22;
const INIT_READ_BIT_POSITION:u8=0b10000000; // reading bits left to right

#[inline]
pub unsafe fn read_bytes<const PIN: u8, const PIN_NUMBER: u8, const TIMER: u8, const DATA_LEN: usize>(data:&mut [u8])->Result<u8, ReadError>{
    let [high_addr, low_addr] = (data.as_ptr() as u16).to_be_bytes(); // 3c
    let mut errors:u8;
    let mut bytes_read_or_additional_error_information=0u8;
    asm!{
        "ld {current_byte} z",
        "ldi {read_bit_position} {init_read_bit_position}", // reading bits left to right
        "ldi {timer_reset_value} 0",
        // wait for low
        "2:",
            "sbic {pin} {pin_number}",
            "rjmp 2b",

        // there should be at least 13 cycles here to do some memory management
        "out {timer_counter_register} {timer_reset_value}", // 21c worst case -> 11 cycles remaining until high is expected
        "cpi {read_bit_position} 0",
        "brne 0f",
        "st z+ {current_byte}",
        "inc {bytes_read}",
        "ldi {read_bit_position} {init_read_bit_position}",
        "cpi {bytes_read} {data_len}", // ensure we're not reading beyond our memory
        "breq 99f",
        "ld {current_byte} z", // else load byte
        // end of the stuff that might be tricky to do
        // in the last high microsecond of a logic 0

        // check for high again
        "0:",
            "sbis {pin} {pin_number}",
            "rjmp 0b",
        // check time sample
        "in {low_time_register} {timer_counter_register}", // 5c into microsecond worst case -> 11 cycles remaining
        "cpi {low_time_register} {min_for_low}",
        "brge 0f",
        "cpi {low_time_register} {max_for_high}",
        "brlo 1f",
        "rjmp 100f",  // not high, not low, prolly controller stop bit...
        // store time sample
        "0:",
            "com {read_bit_position}",
            "and {current_byte} {read_bit_position}", // unset bit at read-bit-position
            "com {read_bit_position}",
            "lsr {read_bit_position}",
            "rjmp 2b",
        "1:",
            "or {current_byte} {read_bit_position}",
            "lsr {read_bit_position}",
            "rjmp 2b",

        // errors and exit
        "99:",
            "ldi {errors} 99",
            "rjmp 101f",
        "100:",
            "ldi {errors} 0",
        "101:",
        read_bit_position=out(reg) _,
        bytes_read=inout(reg) bytes_read_or_additional_error_information,
        errors=out(reg) errors,
        low_time_register=out(reg) _,
        timer_reset_value=out(reg) _,
        current_byte=out(reg) _,
        pin=const PIN,
        pin_number=const PIN_NUMBER,
        timer_counter_register=const TIMER,
        data_len=const DATA_LEN,
        init_read_bit_position=const INIT_READ_BIT_POSITION,
        in("ZL") low_addr,
        in("ZH") high_addr,
        min_for_low=const MINIMUM_LOW_CYCLES_FOR_0,
        max_for_high=const MAXIMUM_LOW_CYCLES_FOR_1,
    }
    match errors{
        0=>Ok(bytes_read_or_additional_error_information),
        99=>Err(ReadError::OutOfMemory(bytes_read_or_additional_error_information)),
        _=>Err(ReadError::UnknownError(errors)),
    }
}