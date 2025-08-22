use ufmt::{derive::uDebug, uDebug, uDisplay};

pub struct N64ControllerState(u32);
#[derive(uDebug)]
pub enum ReadError{
    Failed
}
impl uDisplay for ReadError{
    fn fmt<W>(&self, fmt: &mut ufmt::Formatter<'_, W>) -> Result<(), W::Error>
    where
        W: ufmt::uWrite + ?Sized {
        uDebug::fmt(&self, fmt)
    }
}

pub mod commands{
    pub const POLL_SIGNAL:u8 = 0b00000001u8;
}