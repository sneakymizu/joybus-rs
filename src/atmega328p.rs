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
pub enum ReadError{ // prolly yagni
    WeWentTooFar,
    WaitForHighTimeout,
    WaitForLowTimeout,
    UnknownError
}
impl From<u8> for ReadError{

    fn from(value: u8) -> Self {
        match value{
            UNEXPECTED_CARRY_BIT=>ReadError::WeWentTooFar,
            WAIT_FOR_HIGH_TIMEOUT_ERR=>ReadError::WaitForHighTimeout,
            WAIT_FOR_LOW_TIMEOUT_ERR=>ReadError::WaitForLowTimeout,
            _=>ReadError::UnknownError
        }
    }
}
const UNEXPECTED_CARRY_BIT: u8=0;
const WAIT_FOR_HIGH_TIMEOUT_ERR: u8=1;
const WAIT_FOR_LOW_TIMEOUT_ERR: u8=2;
const SIGNAL_TIMEOUT_IN_CYCLES: u8=8;

#[inline]
pub fn read_bytes<const PIN:u8, const PIN_NUMBER: u8>()->Result<[u8;4], ReadError>{
    let mut reg0:u8 = 0;
    let mut reg1:u8 = 0;
    let mut reg2:u8 = 0;
    let mut reg3:u8 = 0;
    let mut errors:u8=0;
    let mut bit_count:u8=0;
    unsafe {
        asm!{
            // sync with signal
            "ldi {read_byte} 0", // 1c
            "ldi {tmp} 4", // 1c
            "0:", // 1 loop -> 4c
                "dec {tmp}", // 1c
                "sbic {pin} {pin_number}", // 1c/2c
                "brne 0b", // 2c/1c
                "breq 100f", // 1c
            // 2c to 6c off down signal | exits after 5c, 9c, 13c, 17c
            "ldi {tmp} 5",
            "0:", // run to next sampling window
                "dec {tmp}",
                "brne 0b",
            // read bits
            "2:",
                "bst {pin} {pin_number}",
                "lsl {read_byte}",
                "bld {read_byte} 0",
                "ldi {tmp} 4",
                "0:",
                    "dec {tmp}",
                    "brne 0b",
                "nop", // 13c
                "bst {pin} {pin_number}",
                "lsl {read_byte}",
                "bld {read_byte} 0",
                "ldi {tmp} 4",
                "0:",
                    "dec {tmp}",
                    "brne 0b",
                "nop", // 13c
                "bst {pin} {pin_number}",
                "lsl {read_byte}",
                "bld {read_byte} 0",
                "nop",
                "nop",
                "nop", // 6c into bit
                "cpi {read_byte} 3",
                "breq 1f",
                "cpi {read_byte} 1",
                "breq 0f",
                "cpi {read_byte} 2",
                "breq 100f",
                "1:",
                    "lsl {reg0}",
                    "nop",
                    "ori {reg0} 1",
                    "jmp 2b",
                "0:",
                    "lsl {reg0}",
                    "nop",
                    "jmp 2b",
            "98:",
                "ldi {errors} 1",
            "100:",
            pin=const PIN,
            pin_number=const PIN_NUMBER,
            read_byte=out(reg) _,
            reg0=out(reg) reg0,
            tmp=out(reg) _,
            errors=out(reg) errors,
        }
    }
    if errors>0{
        Err(errors.into())
    }else{
        Ok([reg3, reg2, reg1, reg0])
    }
}