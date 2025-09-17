#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use panic_halt as _;
use ufmt::uwriteln;

use joybus_atmega328p::{read_bytes, send_byte, ReadError};

use joybus_types::N64ControllerState;

enum Either<L, R> {
    Left(L),
    Right(R),
}
impl<L, R> Either<L, R> {
    fn left(self) -> L {
        match self {
            Either::Left(l) => l,
            Either::Right(_r) => panic!(),
        }
    }
    fn right(self) -> R {
        match self {
            Either::Left(_l) => panic!(),
            Either::Right(r) => r,
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let timer = dp.TC0;
    // normal operating timer
    timer.tccr0a.reset();
    timer.tccr0b.write(|w| w.cs0().direct()); // no prescale, normal timer operation
    let pwm_sound_driver = dp.TC1;
    pwm_sound_driver
        .tccr1a
        .write(|w| w.com1a().match_toggle().wgm1().bits(1));
    pwm_sound_driver
        .tccr1b
        .write(|w| w.wgm1().bits(0b10).cs1().direct());

    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut _reader_pin = Either::Left(pins.d6.into_output_high().downgrade());
    pins.d9.into_output(); // oc1a is pb1, which is d9 on arduino nano - setting high for pwm output

    arduino_hal::delay_ms(3000);
    led_pin.set_low();
    _reader_pin = Either::Right(_reader_pin.left().into_pull_up_input());
    const DATA_LEN: usize = 4;
    let mut data = [0u8; DATA_LEN];
    loop {
        _reader_pin = Either::Left(_reader_pin.right().into_output_high());
        unsafe { send_byte::<0x0b, 0x06, 1>([joybus_types::commands::POLL_SIGNAL]) };
        _reader_pin = Either::Right(_reader_pin.left().into_pull_up_input());
        let _ = match unsafe { read_bytes::<0x9, 0x6, 0x26, 0x15, 1, DATA_LEN>(&mut data) } {
            Ok(b) => uwriteln!(serial, "(Stop-bit) Bytes are {:?} {:?}\r", b, data),
            Err(ReadError::OutOfMemory(len)) => {
                uwriteln!(serial, "(No Stopbit) Bytes are {:?}: {:?}\r", len, data)
            }
            Err(e) => {
                let _ = uwriteln!(serial, "Got error {:?}\r", e);
                continue;
            }
        };

        let state: N64ControllerState = data.into();
        let power: Option<i8> = if state.c_right() {
            // A
            Some(0)
        } else if state.c_down() {
            // F
            Some(-4)
        } else if state.a_button() {
            // D
            Some(-7)
        } else if state.c_left() {
            // B
            Some(2)
        } else if state.c_up() {
            // D
            Some(5)
        } else {
            None
        };
        if power.is_none() {
            pwm_sound_driver.ocr1a.write(|w| w.bits(0));
            continue;
        }
        let power = power.unwrap() as i8
            - state.z_button() as i8  // augments half step down
            + state.y_axis().signum() * 2 // augments a whole step
            + state.right_trigger() as i8; // augments half step up
        let freq = calculate_equal_temperate_frequency::<440>(power);
        let top = frequency_into_top(freq);
        pwm_sound_driver.ocr1a.write(|w| w.bits(top));
    }
}

const SEMITONE_FACTOR: f32 = 1.05946309436;
fn calculate_equal_temperate_frequency<const BASE_FREQUENCY: u32>(power: i8) -> u16 {
    let mut frequency = BASE_FREQUENCY as f32;
    let fun = if power >= 0 {
        <f32 as core::ops::Mul>::mul
    } else {
        <f32 as core::ops::Div>::div
    };
    for _ in 0..power.abs() {
        frequency = fun(frequency, SEMITONE_FACTOR);
    }
    (frequency + 0.5) as u16
}

fn frequency_into_top(freq: u16) -> u16 {
    // only works for 16 bit phase and frequency correct timer
    let res = 16000000 / (4 * freq as u32);
    res as u16
}
