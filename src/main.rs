#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use panic_halt as _;
use ufmt::uwriteln;

use joybus_atmega328p::{read_bytes, send_byte, ReadError};

use joybus_types::N64ControllerState;

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
    const DATA_LEN:usize=4;
    let mut data = [0u8;DATA_LEN];
    loop {
        _reader_pin = Either::Left(_reader_pin.right().into_output_high());
        send_byte::<0x0b, 0x06>(joybus_types::commands::POLL_SIGNAL);
        _reader_pin = Either::Right(_reader_pin.left().into_pull_up_input());
        let _ = match read_bytes::<0x9, 0x6, 0x26, DATA_LEN>(&mut data){
            Ok(b) => uwriteln!(serial, "(Stop-bit) Bytes are {:?} {:?}\r", b, data),
            Err(ReadError::OutOfMemory(len)) => uwriteln!(serial, "(No Stopbit) Bytes are {:?}: {:?}\r", len, data),
            Err(e) => {
                let _ = uwriteln!(serial, "Got error {:?}\r", e);
                continue;
            },
        };
        let state: N64ControllerState = data.into();
        let _ = uwriteln!(serial, "A is {}\r", if state.a_button(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "B is {}\r", if state.b_button(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "Z is {}\r", if state.z_button(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "C up is {}\r", if state.c_up(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "C down {}\r", if state.c_down(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "C left {}\r", if state.c_left(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "C right {}\r", if state.c_right(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "Reset is {}\r", if state.reset(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "Start is {}\r", if state.start_button(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "Right trigger is {}\r", if state.right_trigger(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "Left trigger {}\r", if state.left_trigger(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "Dpad up is {}\r", if state.dpad_up(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "Dpad down is {}\r", if state.dpad_down(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "Dpad left is {}\r", if state.dpad_left(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "Dpad right is {}\r", if state.dpad_right(){"pressed"}else{"released"});
        let _ = uwriteln!(serial, "X is {}\r", state.x_axis());
        let _ = uwriteln!(serial, "Y is {}\r", state.y_axis());

        arduino_hal::delay_ms(500);
    }
}
