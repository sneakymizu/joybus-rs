#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(abi_avr_interrupt)]

use crate::same_pin_io::SwitchablePin;
use arduino_hal::{pac::{tc0::tifr0::OCF0A_W, TC0}, port::{
    mode::{Input, Output, PullUp},
    Pin, PinOps,
}, Peripherals};
use avr_device::{generic::Reg, interrupt::{CriticalSection, Mutex}};
use embedded_hal::timer::Periodic;
use core::{arch::asm, mem};
use panic_halt as _;
use ufmt::uwriteln;

mod same_pin_io;

const READ_COMMAND: u16 = 0b11;
const READ_COMMAND_LENGTH: u8 = 9;
const STATE_RESPONSE_LENGTH: u8 = 32;
/*
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
        state.peripherals.borrow(cs).write(reg_val^2);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    };
}
fn do_interrupts(){
    dp.TC0.timsk0.write(|w|w.ocie0a().set_bit());
    let ocra0 = Mutex::new(dp.TC0.ocr0a.as_ptr());
    unsafe {
        INTERRUPT_STATE = mem::MaybeUninit::new(InterruptState{peripherals: ocra0});
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
    unsafe {avr_device::interrupt::enable();}
}
*/

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led_pin = pins.d13.into_output();
    pins.d6.into_output_high();
    dp.TC0.tccr0a.write(|w|w.wgm0().pwm_phase().com0a().match_toggle());
    dp.TC0.tccr0b.write(|w|w.cs0().prescale_8().wgm02().set_bit()); // set wgm02 for fastpwm with top = OCRA
    dp.TC0.ocr0a.write(|w|w.bits(1)); // set OCRA = 16 for 1µs tick or prescale 8 for nums in µs
    
    led_pin.set_high();


    //let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    //uwriteln!(serial, "Lets go\r").unwrap();
    led_pin.set_low();
    
    loop {
        if dp.TC0.tifr0.read().ocf0a().bit_is_set(){
            dp.TC0.ocr0a.modify(|r,w|{
                w.bits(r.bits()^2)
            });
        }
    }
}
