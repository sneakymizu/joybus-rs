use core::arch::asm;

// assumes the given port is configured as output
// designed for atmega running with 16MHz clock
// 1µs is 16 clock cylces, 3µs is 48
const BIT_COUNTER_INIT: u8 = 128u8;
#[inline]
pub unsafe fn send_bytes<const PORT: u8, const PIN_NUMBER: u8>(bytes: &[u8]) {
    let bytes_to_send = bytes.len() as u8;
    let [high_addr, low_addr] = (bytes.as_ptr() as u16).to_be_bytes(); // 3c
    asm! {
        "mov {byte_counter} {bytes_to_send}",
        "ldi {bit_counter} {bit_counter_init}",
        "ld {input} z+",
        "clz",
        // this setup should take too long to properly align, but a button detection functiones well
        "rjmp 5f",
        "2:",
            "lsr {bit_counter}", // 1c
        "5:",
            "cbi {port}, {pin}", // 2c
            "breq 3f", // 1c/2c
            "nop",
            "nop",
            "nop",
            "nop",
            "rjmp 4f",
        "3:", // check bytes and bits to send
            "ld {input} z+", // 2c
            "ldi {bit_counter} {bit_counter_init}", //1c
            "dec {byte_counter}", // 1c
            "breq 100f", // 1c/2c
        "4:", // 8c since low | check if 0 or 1 is sent
            "mov {tmp} {input}", // 1c
            "and {tmp} {bit_counter}", // 1c
            "breq 0f", // 2c for send 0, 1c for send 1
        "1:", // enter here with 11c (send 1)
            // start waiting rest of high for logic one here
            "nop", // 1c
            "nop", // 1c
            "nop", // 1c // 14c since low
            "sbi {port} {pin}", // 2c high on cycle 16
            // now keep it high for 4c less than 3µs -> 44 cycles
            "ldi {tmp} 14", // 1c
            "1:",
                "dec {tmp}", // 1c
                "brne 1b",  // 2c on branch else 1c
                // exit with 42c high
            "nop",
            "rjmp 2b", // rjmp back with 45 cycles (bit shift after each bit) so low on cycle 48 since setting high -> logic one
        "0:", // enter here with 12c (send 0)
            // keep low for additional 2µ (->32 cycles) + 2 cycles, set high and jump back on cycle 12 of high signal
            "nop", // 1c
            "ldi {tmp}, 11", // 1c
            "1:",
                "dec {tmp}", // 1c
                "brne 1b", // 1c/2c
                // exit with 46 cycles
            "sbi {port}, {pin}", // 2c - high on cycle 48 since low
            "ldi {tmp}, 3", // 1c
            "1:",
                "dec {tmp}", // 1c
                "brne 1b", // 1c
                // exit with 9 cycles high
            "nop",
            "nop",
            "rjmp 2b", // 2c // jump back with 13 (bit shift after each bit) cycles high
        "100:", // send stop bit (starts with 9c low)
            "nop", // 1c
            "nop", // 1c
            "nop", // 1c
            "nop", // 1c
            "nop", // 1c
            "sbi {port}, {pin}", // 2c - high on cycle 16
            // no need to count here anymore, let the other side handle the response
        bit_counter=out(reg) _,
        byte_counter=out(reg) _,
        input=out(reg) _,
        tmp=out(reg) _,
        in("ZL") low_addr,
        in("ZH") high_addr,
        bytes_to_send=in(reg) bytes_to_send,
        bit_counter_init=const BIT_COUNTER_INIT,
        port=const PORT,
        pin=const PIN_NUMBER, // should be the same for port, ddr and pmsk
    }
}

#[cfg_attr(feature = "ufmt", derive(uDebug))]
pub enum ReadError {
    OutOfMemory(u8),
    Timeout(u8),
    UnknownError(u8),
}

// each loop for pin check might exit with 4~5 cycles wasted & third party controllers aren't too specific about timing so this expects more than "stopbit"-low
// seemingly perfect stop bit timing alignment with loading timer value is 33 cycles. So taking the 32 low cycles for stop bit + 4~5 misalignment cycles should be greater or equal to ~37
const MINIMUM_LOW_CYCLES_FOR_0: u8 = 37;
// 16+5 cycles, compares against lower
const MAXIMUM_LOW_CYCLES_FOR_1: u8 = 22;
const INIT_READ_BIT_POSITION: u8 = 0b10000000; // reading bits left to right

#[inline]
pub unsafe fn read_bytes<
    const PIN: u8,
    const PIN_NUMBER: u8,
    const TIMER_VALUE_REGISTER: u8,
    const TIMER_MATCH_REGISTER: u8,
    const TIMER_MATCH_NUMBER: u8,
>(
    data: &mut [u8],
) -> Result<u8, ReadError> {
    let data_len = data.len() as u8;
    let [high_addr, low_addr] = (data.as_ptr() as u16).to_be_bytes(); // 3c
    let mut errors: u8;
    let mut bytes_read_or_additional_error_information = 0u8;
    asm! {
        "ld {current_byte} z", // 2c
        "ldi {read_bit_position} {init_read_bit_position}", // 1c | reading bits left to right
        "ldi {timer_reset_value} 0", // 1c
        "out {timer_counter_register} {timer_reset_value}", // 1c | ensure timer is reset
        "sbi {timer_match_register} {timer_match_position}", // 2c
        // wait for low
        "2:",
            "sbic {timer_match_register} {timer_match_position}",
            "rjmp 98f",
            "sbic {pin} {pin_number}",
            "rjmp 2b",

        // there should be at least 11 cycles here to do some memory management
        // misalignment possible as this does not include anything before asm-start
        "out {timer_counter_register} {timer_reset_value}", // 1c | 7c into low worst case -> 9 cycles after this remaining until high is expected
        "cpi {read_bit_position} 0", // 1c
        "brne 0f", // 1c/2c | skip (2c) for still on same byte
        "st z+ {current_byte}", // 2c
        "inc {bytes_read}", // 1c
        "ldi {read_bit_position} {init_read_bit_position}", // 1c
        "cp {bytes_read} {data_len}", // 1c | ensure we're not reading beyond our memory
        "breq 99f", // 1c/2c | exit on out of memory
        "ld {current_byte} z", // 2c | else load byte
        // about 8 cycles since timer check.
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
        "98:", // Timeout while wait for low signal
            "ldi {errors} 98",
            "in {bytes_read} {timer_counter_register}",
            "rjmp 101f",
        "99:", // end of memory error
            // to verify if stop bit was sent this will wait for high again
            "0:",
                // NOTE: this does not check for timeouts as the protocol requires a passive pullup.
                // any component would need to actively keep pulling down the signal.
                "sbis {pin} {pin_number}",
                "rjmp 0b",
            "ldi {errors} 99", // set out of memory error value
            "in {bytes_read} {timer_counter_register}", // add timeing information for further evaluation of stop bit timing
            "rjmp 101f",
            // exit with success, overwriting the error value as the read bit was the stop bit
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
        timer_counter_register=const TIMER_VALUE_REGISTER,
        timer_match_register=const TIMER_MATCH_REGISTER,
        timer_match_position=const TIMER_MATCH_NUMBER,
        data_len=in(reg) data_len,
        init_read_bit_position=const INIT_READ_BIT_POSITION,
        in("ZL") low_addr,
        in("ZH") high_addr,
        min_for_low=const MINIMUM_LOW_CYCLES_FOR_0,
        max_for_high=const MAXIMUM_LOW_CYCLES_FOR_1,
    }
    match errors {
        0 => Ok(bytes_read_or_additional_error_information),
        99 => {
            if MAXIMUM_LOW_CYCLES_FOR_1 < bytes_read_or_additional_error_information
                && bytes_read_or_additional_error_information < MINIMUM_LOW_CYCLES_FOR_0
            {
                Ok(data_len)
            } else {
                Err(ReadError::OutOfMemory(
                    bytes_read_or_additional_error_information,
                ))
            }
        }
        98 => Err(ReadError::Timeout(
            bytes_read_or_additional_error_information,
        )),
        _ => Err(ReadError::UnknownError(errors)),
    }
}
