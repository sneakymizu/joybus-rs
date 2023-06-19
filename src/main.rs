#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::Peripherals;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    dp.PORTD
        .ddrd
        .modify(|r, w| unsafe { w.bits(r.bits() | 1 << 7) });
    loop {}
}
