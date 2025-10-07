use arduino_hal::port::{
    mode::{Floating, Input},
    Pin, PinOps,
};

use crate::{joybus::JoybusPinRead, read_bytes, send_byte, ReadError};

pub type JoybusPinWrapping<PIN> = Pin<Input<Floating>, PIN>;
pub(super) struct JoybusPinWrapper<PIN: PinOps> {
    input: Option<Pin<Input<Floating>, PIN>>,
}
impl<PIN: PinOps> JoybusPinWrapper<PIN> {
    pub fn from_pin(pin: Pin<Input<Floating>, PIN>) -> Self {
        Self { input: Some(pin) }
    }
}

// PORTB
impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PB0> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x00>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x00, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PB1> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x01>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x01, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PB2> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x02>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x02, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PB3> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x03>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x03, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PB4> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x04>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x04, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PB5> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x05>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x05, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PB6> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x06>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x06, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PB7> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x07>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x7, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

// PORTC
impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PC0> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x00>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x6, 0x00, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PC1> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x01>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x6, 0x01, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PC2> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x02>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x6, 0x02, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PC3> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x03>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x6, 0x03, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PC4> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x04>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x6, 0x04, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PC5> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x05>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x6, 0x05, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PC6> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x06>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x6, 0x06, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

// PORTD
impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PD0> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x00>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x00, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PD1> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x01>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x01, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PD2> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x02>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x02, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PD3> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x03>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x03, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PD4> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x04>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x04, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PD5> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x05>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x05, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PD6> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x06>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x06, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead for JoybusPinWrapper<::arduino_hal::hal::port::PD7> {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x07>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x9, 0x07, 0x26, 0x15, 1>(recv) }.map(|v| v as usize)
    }
}
