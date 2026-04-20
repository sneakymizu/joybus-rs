/// Trait should supply a configuration allowing a timer to count each clock cycle.
pub trait TimerConfigurator {
    /// Should configure the timer to count cycles for the chosen hardware.
    /// This allows the implementation to extrapolate timing from cycle counts.
    fn configure_count_cycles(&mut self);
    /// Should configure a timeout for the timer.
    /// If the timeout value is reached the impelementation should assume that the communication partner will not respond.
    fn configure_timeout(&mut self, timeout: usize);
}

impl TimerConfigurator for ::arduino_hal::pac::TC0 {
    fn configure_count_cycles(&mut self) {
        // setup timer for joybus readings
        // normal operating timer
        self.tccr0a().reset();
        self.tccr0b().write(|w| w.cs0().direct()); // no prescale, normal timer operation
    }
    fn configure_timeout(&mut self, timeout: usize) {
        self.ocr0a().write(|w| unsafe{w.bits(timeout as u8)});
    }
}
