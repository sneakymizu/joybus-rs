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
    Timeout,
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
pub fn read_bytes<const PIN:u8, const PIN_NUMBER: u8>(data:&mut [u8;4])->Result<u8, ReadError>{
    let [high_addr, low_addr] = (data.as_ptr() as u16).to_le_bytes();
    let mut errors:u8;
    let mut byte_sampling:u8;
    let mut bytes_read:u8;
    unsafe {
        asm!{
            // sync with signal
            "ldi {bytes_read} 0",
            "ldi {tmp} 12",
            "0:", // 1 loop -> 4c
                "dec {tmp}", // 1c
                "sbic {pin} {pin_number}", // 1c/2c
                "brne 0b", // 2c/1c
                "breq 98f", // 1c
            "ldi {tmp} 4", // 1c
            "0:",
                "dec {tmp}", // 1c
                "brne 0b", // 2c
            "2:",
                "sbis {pin} {pin_number}", // 1c/2c/3c
                "rjmp 0f", // 2c
                "rjmp 1f", // 2c
                "0:",
                    "ldi {byte_sampling} 0", // 1c
                    "rjmp 3f", // 2c
                "1:",
                    "ldi {byte_sampling} 0b10000000", // 1c
                    "nop",
                "3:", // incomming jumps with 6c
                    "ldi {tmp} 3",
                    "0:",
                        "dec {tmp}",
                        "brne 0b",
                // -> end of block with 15c

                "sbic {pin} {pin_number}", // 1c
                "ori {byte_sampling} 0b00000001", // 1c
                "ldi {tmp} 4", // 1c
                "0:",
                    "dec {tmp}", // 1c
                    "brne 0b", // 1c/2c
                "mov {current_sample_value} {byte_sampling}", // 1c
                "inc {bytes_read}", // 1c -> end of block with 15c

                "sbic {pin} {pin_number}", // 1c
                "ori {byte_sampling} 0b00001000", // 1c
                "ld {tmp} z", // 2c
                "lsl {byte_sampling}", // 1c
                "brcs 1f", // 1c/2c  (7th bit set -> 1)
                "brhs 0f", // 1c/2c  (3rd bit set -> 0)

                // stop bit or error (0 bit set or no bits -> stop/error)
                "andi {byte_sampling} 0b10", // original stop bit marker was shifted
                "breq 99f",  // and 1 is 0 -> stop bit was not set
                "jmp 100f", // 2c
                "1:",
                    "lsl {tmp}", // 1c
                    "ori {tmp} 1", // 1c
                    "st z {tmp}", // 2c
                    "brcc 1f", //1c/2c
                    "adiw ZH:ZL 1", // 2c
                    "brne 2b", // 2c
                    "1:",
                    "jmp 2b", // 3c (9c on exit 15c since reading bit)
                "0:",
                    "lsl {tmp}", // 1c
                    "st z {tmp}", // 2c
                    "brcc 1f", //1c/2c
                    "adiw ZH:ZL 1", // 2c
                    "brne 2b", // 2c
                    "1:",
                    "jmp 2b", // 3c (8c on exit 15c since reading bit)
            "98:",
                "ldi {errors} 2",
                "jmp 101f",
            "99:",
                "mov {errors} 1",
                "jmp 101f",
            "100:",
                "ldi {errors} 0",
            "101:",
            pin=const PIN,
            pin_number=const PIN_NUMBER,
            byte_sampling=out(reg) _,
            current_sample_value=out(reg) byte_sampling,
            bytes_read=out(reg) bytes_read,
            in("ZL") low_addr,
            in("ZH") high_addr,
            tmp=out(reg) _,
            errors=out(reg) errors,
        }
    }
    match errors{
        0=>Ok(bytes_read),
        1=>Err(ReadError::StopConditionMissmatch(byte_sampling)),
        2=>Err(ReadError::Timeout),
        _=>Err(ReadError::UnknownError(errors)),
    }
}