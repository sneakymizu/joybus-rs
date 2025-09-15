#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]

use arduino_hal::pac::TC1;
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
struct FrequencyGenerator<const PRESCALER: u16>{
    pwm_driver: TC1,
}
impl<const PRESCALER:u16> FrequencyGenerator<PRESCALER>{
    const FREQUENCY_CONVERSION_FACTOR:u16=(16000000/(2*PRESCALER as u32)) as u16;

    fn set_freq(&mut self, freq:u16){
        let top = if freq == 0{
            0
        }else{
            Self::FREQUENCY_CONVERSION_FACTOR*freq
        };
        self.pwm_driver.ocr1a.write(|w|w.bits(top));
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let timer = dp.TC0;
    // normal operating timer
    timer.tccr0a.reset();
    timer.tccr0b.write(|w|w.cs0().direct());  // no prescale, normal timer operation
    let pwm_sound_driver = dp.TC1;
    pwm_sound_driver.tccr1a.write(|w|w.com1a().match_toggle().wgm1().bits(1));
    pwm_sound_driver.tccr1b.write(|w|w.wgm1().bits(2).cs1().prescale_1024());
    let mut generator = FrequencyGenerator::<1024>{
        pwm_driver: pwm_sound_driver,
    };

    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    led_pin.set_high();

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut _reader_pin = Either::Left(pins.d6.into_output_high().downgrade());
    generator.set_freq(0);
    pins.d9.into_output();  // oc1a is pb1, which is d9 on arduino nano - setting high for pwm output

    //uwriteln!(serial, "Lets go\r").unwrap();
    led_pin.set_low();
    arduino_hal::delay_ms(3000);
    _reader_pin = Either::Right(_reader_pin.left().into_pull_up_input());
    const DATA_LEN:usize=4;
    let mut data = [0u8;DATA_LEN];
    loop {
        _reader_pin = Either::Left(_reader_pin.right().into_output_high());
        unsafe {send_byte::<0x0b, 0x06, 1>([joybus_types::commands::POLL_SIGNAL])};
        _reader_pin = Either::Right(_reader_pin.left().into_pull_up_input());
        let _ = match unsafe{read_bytes::<0x9, 0x6, 0x26, 0x15, 1, DATA_LEN>(&mut data)}{
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
