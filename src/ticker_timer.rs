use alloc::boxed::Box;
use core::cell::RefCell;
use core::time::Duration;
use arduino_hal::port::PinOps;
use avr_device::interrupt::Mutex;
use crate::same_pin_io::SwitchablePin;

const MICROSECOND_TICKS: u8 = 16;
static mut LOAD_AND_STORE: Mutex<RefCell<LoadAndStore<PIN>>> = Mutex::new(RefCell::default());

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA(){

}
struct Foo{
    next_tick: u8,
    load_and_store: LoadAndStore<>
}

enum N64PinData{
    High,
    Low
}
impl N64PinData{
    fn duration(&self)->(u8, u8){
        match self{
            N64PinData::High => (3,1),
            N64PinData::Low => (1,3)
        }
    }
}

enum Data{
    Send,
    Read(u8)
}

pub struct LoadAndStore<PIN: PinOps>{
    pin: SwitchablePin<PIN>,
    data: Data
}
impl<PIN: PinOps> LoadAndStore<PIN>{
    fn store(&self){

    }
    fn load(&self){

    }
}

static mut COUNTER: u8 = 0;
pub trait TickerTimer{
    fn configure_duration(unit: Duration)->Self;
    fn reset(&self);
    fn current_ticks(&self) -> u8;
}

pub trait InterruptTimer{
    fn configure_ticks(&self, call);
    fn current_ticks(&self) -> u8;
}

pub trait Resettable{
    fn reset(&self);
}