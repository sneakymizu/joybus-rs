#![no_std]


pub struct N64ControllerState([u8;4]);
impl From<[u8;4]> for N64ControllerState{
    fn from(value: [u8;4]) -> Self {
        N64ControllerState(value)
    }
}
impl From<u32> for N64ControllerState{
    fn from(value: u32)->Self{
        N64ControllerState(value.to_le_bytes())
    }
}
impl N64ControllerState{
    pub fn a_button(&self)->bool{
        (self.0[0]&128u8) != 0
    }
    pub fn b_button(&self)->bool{
        (self.0[0]&64u8) != 0
    }
    pub fn z_button(&self)->bool{
        (self.0[0]&32u8) != 0
    }
    pub fn start_button(&self)->bool{
        (self.0[0]&16u8) != 0
    }
    pub fn dpad_up(&self)->bool{
        (self.0[0]&8u8) != 0
    }
    pub fn dpad_down(&self)->bool{
        (self.0[0]&4u8) != 0
    }
    pub fn dpad_left(&self)->bool{
        (self.0[0]&2u8) != 0
    }
    pub fn dpad_right(&self)->bool{
        (self.0[0]&1u8) != 0
    }
    pub fn reset(&self)->bool{
        (self.0[1]&128u8)!=0
    }
    pub fn left_trigger(&self)->bool{
        (self.0[1]&32u8)!=0
    }
    pub fn right_trigger(&self)->bool{
        (self.0[1]&16u8)!=0
    }
    pub fn c_up(&self)->bool{
        (self.0[1]&8u8)!=0
    }
    pub fn c_down(&self)->bool{
        (self.0[1]&4u8)!=0
    }
    pub fn c_left(&self)->bool{
        (self.0[1]&2u8)!=0
    }
    pub fn c_right(&self)->bool{
        (self.0[1]&1u8)!=0
    }
    pub fn x_axis(&self)->i16{
        self.0[2] as i16
    }
    pub fn y_axis(&self)->i16{
        self.0[3] as i16
    }
}

pub mod commands{
    pub const POLL_SIGNAL:u8 = 0b00000001u8;
}