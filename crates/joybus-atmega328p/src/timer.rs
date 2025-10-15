/// Trait should supply a configuration allowing a timer to count each clock cycle.
pub trait TimerConfigurator {
    fn configure_count_cycles(&mut self);
    fn configure_timeout(&mut self, timeout: usize);
}

impl TimerConfigurator for ::arduino_hal::pac::TC0 {
    fn configure_count_cycles(&mut self) {
        // setup timer for joybus readings
        // normal operating timer
        self.tccr0a.reset();
        self.tccr0b.write(|w| w.cs0().direct()); // no prescale, normal timer operation
    }
    fn configure_timeout(&mut self, timeout: usize) {
        self.ocr0a.write(|w| w.bits(timeout as u8));
    }
}
