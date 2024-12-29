#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]

use arduino_hal::pac::tc0::TIFR0;
use avr_device::generic::{Readable, Reg, RegisterSpec, Writable};
use panic_halt as _;

//mod same_pin_io;

const READ_COMMAND: u16 = 0b11;
const READ_COMMAND_LENGTH: u8 = 9;
const STATE_RESPONSE_LENGTH: u8 = 32;

/* // interrupts have too much overhead with ops, jumps and guardrails added by the compiler, prolly better to have read/write in code directly
struct InterruptState {
    peripherals: Mutex<*mut u8>,
}

static mut INTERRUPT_STATE: mem::MaybeUninit<InterruptState> = mem::MaybeUninit::uninit();

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA(){
    let state = unsafe {
        INTERRUPT_STATE.assume_init_ref()
    };
    unsafe {
        let cs = CriticalSection::new();
        let reg_val = state.peripherals.borrow(cs).read();
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        state.peripherals.borrow(cs).write(reg_val^4);
    }
}
fn setup_interrupt_data(dp: &Peripherals){
    dp.TC0.timsk0.write(|w|w.ocie0a().set_bit());
    let ocra0 = Mutex::new(dp.TC0.ocr0a.as_ptr());
    unsafe {
        INTERRUPT_STATE = mem::MaybeUninit::new(InterruptState{peripherals: ocra0});
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
*/

fn write_on_tick<REG: Readable + Writable>(timer: &impl Timing, bit_sequence: u16, out: &Reg<REG>)
where
    REG: RegisterSpec<Ux = u8>,
{
    if let Some(_t) = timer.with_int() {
        out.modify(|r, w| unsafe {
            let dat = r.bits();
            w.bits(dat ^ 2)
        });
    }
}

trait Timing {
    fn with_int(&self) -> Option<TimedSection>;
}

struct Tim<'a>(&'a TIFR0);
struct TimedSection<'a>(&'a TIFR0);
impl<'a> Drop for TimedSection<'a> {
    fn drop(&mut self) {
        self.0.write(|w| w.ocf0a().set_bit());
    }
}
impl<'a> Timing for Tim<'a> {
    fn with_int(&self) -> Option<TimedSection> {
        if self.0.read().ocf0a().bit_is_set() {
            Some(TimedSection(&self.0))
        } else {
            None
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output_high();

    // D6 is OCA0 which will be toggled by timer0
    pins.d6.into_output_high();
    arduino_hal::delay_ms(1000);
    dp.TC0
        .tccr0a
        .write(|w| w.wgm0().pwm_fast().com0a().match_toggle());
    dp.TC0
        .tccr0b
        .write(|w| w.cs0().prescale_8().wgm02().set_bit()); // set wgm02 for OCRA = top
    dp.TC0.ocr0a.write(|w| w.bits(5)); // set OCRA = 15 for 1µs tick or prescale 8 to have each digit be 0.5 µs (starting at 0=0.5µ 1=1µ..)

    //let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    //uwriteln!(serial, "Lets go\r").unwrap();

    led_pin.set_low();
    let tim = Tim(&dp.TC0.tifr0);
    loop {
        write_on_tick(&tim, READ_COMMAND, &dp.TC0.ocr0a);
    }
}
