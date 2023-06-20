#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::sync::atomic::{AtomicBool, Ordering};
use panic_halt as _;

static PIN_CHANGED: AtomicBool = AtomicBool::new(false);

#[avr_device::interrupt(atmega328p)]
fn PCINT2(){
    PIN_CHANGED.store(true, Ordering::SeqCst);
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
        // Enable the PCINT2 pin change interrupt
    dp.EXINT.pcicr.write(|w| unsafe { w.bits(0b100) });

    // Enable pin change interrupts on PCINT18 which is pin PD2 (= d2)
    dp.EXINT.pcmsk2.write(|w| w.bits(0b100));
    
    //From this point on an interrupt can happen
    unsafe { avr_device::interrupt::enable() };
    loop {}
}
