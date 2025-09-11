#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use panic_halt as _;
use ufmt::uwriteln;

use joybus_atmega328p::{read_bytes, send_byte, ReadError};

mod n64;

enum Either<L,R>{
    Left(L),
    Right(R)
}
impl<L,R> Either<L,R>{
    fn left(self)->L{
        match self{
            Either::Left(l)=>l,
            Either::Right(_r)=>panic!()
        }
    }
    fn right(self)->R{
        match self{
            Either::Left(_l)=>panic!(),
            Either::Right(r)=>r
        }
    }
}
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let timer = dp.TC0;
    // normal operating timer
    timer.tccr0a.reset();
    timer.tccr0b.write(|w|w.cs0().direct());  // no prescale, normal timer operation
    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut _reader_pin = Either::Left(pins.d6.into_output_high().downgrade());

    //uwriteln!(serial, "Lets go\r").unwrap();
    led_pin.set_low();
    arduino_hal::delay_ms(3000);
    _reader_pin = Either::Right(_reader_pin.left().into_pull_up_input());
    loop {
        let mut data = [0u8;4];
        _reader_pin = Either::Left(_reader_pin.right().into_output_high());
        send_byte::<0x0b, 0x06>(n64::commands::POLL_SIGNAL);
        let pin = _reader_pin.left().into_pull_up_input();
        let _ = match read_bytes::<0x9, 0x6, 0x26>(&mut data){
            Ok(b) => uwriteln!(serial, "(Stop-bit) Bytes are {:?} {:?}\r", b, data),
            Err(ReadError::OutOfMemory(len)) => uwriteln!(serial, "(No Stopbit) Bytes are {:?}: {:?}\r", len, data),
            Err(e) => uwriteln!(serial, "Got error {:?}\r", e),
        };
        _reader_pin = Either::Right(pin);
        arduino_hal::delay_ms(100);
    }
}
