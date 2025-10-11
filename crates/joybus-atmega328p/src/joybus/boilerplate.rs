use super::AvrPinDefinition;
use crate::{read_bytes, send_byte, ReadError};
use arduino_hal::port::PinOps;

/// Internal trait to define how a pin can read joybus information.
pub(super) trait JoybusPinRead {
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError>;
}

/// Internal trait to define how the joybus reader wants the timer to be configured.
pub(super) trait TimerConfigurator {
    fn configure(&mut self);
}

pub(super) struct JoybusPinWrapper<PIN: PinOps, TIMER> {
    input: Option<AvrPinDefinition<PIN>>,
    _timer: TIMER, // own a timer here to better constrain the timer usage, making the unsafe code safer
}
impl<PIN: PinOps, TIMER> JoybusPinWrapper<PIN, TIMER> {
    pub fn from_pin_and_timer(pin: AvrPinDefinition<PIN>, timer: TIMER) -> Self {
        Self {
            input: Some(pin),
            _timer: timer,
        }
    }
}

// PORTB
impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PB0, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x00>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x00, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PB1, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x01>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x01, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PB2, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x02>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x02, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PB3, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x03>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x03, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PB4, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x04>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x04, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PB5, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x05>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x05, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PB6, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x06>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x06, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PB7, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x05, 0x07>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x03, 0x07, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

// PORTC
impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PC0, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x00>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x06, 0x00, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PC1, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x01>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x06, 0x01, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PC2, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x02>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x06, 0x02, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PC3, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x03>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x06, 0x03, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PC4, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x04>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x06, 0x04, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PC5, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x05>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x06, 0x05, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PC6, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x08, 0x06>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x06, 0x06, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

// PORTD
impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PD0, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x00>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x09, 0x00, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PD1, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x01>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x09, 0x01, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PD2, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x02>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x09, 0x02, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PD3, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x03>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x09, 0x03, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PD4, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x04>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x09, 0x04, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PD5, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x05>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x09, 0x05, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}

impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PD6, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x06>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x09, 0x06, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}
impl JoybusPinRead
    for JoybusPinWrapper<::arduino_hal::hal::port::PD7, ::arduino_hal::hal::pac::TC0>
{
    fn joybus_read(&mut self, send: &[u8], recv: &mut [u8]) -> Result<usize, ReadError> {
        let output = self.input.take().unwrap().into_output_high();
        unsafe { send_byte::<0x0b, 0x07>(send) };
        self.input = Some(output.into_floating_input());
        unsafe { read_bytes::<0x09, 0x07, 0x26, 0x15, 0x01>(recv) }.map(|v| v as usize)
    }
}
