use arduino_hal::port::mode::{Input, Output, PullUp};
use arduino_hal::port::{Pin, PinOps};

pub struct SwitchablePin<PIN: PinOps> {
    read_pin: Option<Pin<Input<PullUp>, PIN>>,
    write_pin: Option<Pin<Output, PIN>>,
}

impl<PIN: PinOps> SwitchablePin<PIN> {
    pub fn from_output(pin: Pin<Output, PIN>) -> Self {
        SwitchablePin {
            read_pin: None,
            write_pin: Some(pin),
        }
    }
    pub fn as_output(&mut self) -> Option<&mut Pin<Output, PIN>> {
        if self.read_pin.is_some() {
            self.write_pin = Some(self.read_pin.take().unwrap().into_output())
        }
        self.write_pin.as_mut()
    }
    pub fn as_input(&mut self) -> Option<&Pin<Input<PullUp>, PIN>> {
        if self.write_pin.is_some() {
            self.read_pin = Some(self.write_pin.take().unwrap().into_pull_up_input());
        }
        self.read_pin.as_ref()
    }
}
