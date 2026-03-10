// drivers/input/keyboard.rs
use x86_64::instructions::port::Port;
use super::KeyboardDevice;

pub struct Ps2Keyboard {
    data: Port<u8>,
    status: Port<u8>,
}

impl Ps2Keyboard {
    pub const fn new() -> Self {
        Self {
            data: Port::new(0x60),
            status: Port::new(0x64),
        }
    }
}

impl KeyboardDevice for Ps2Keyboard {
    fn read_scancode(&mut self) -> Option<u8> {
        unsafe {
            if self.status.read() & 1 != 0 {
                Some(self.data.read())
            } else {
                None
            }
        }
    }
}
