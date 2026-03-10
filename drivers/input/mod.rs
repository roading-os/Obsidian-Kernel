// drivers/input/mod.rs
use spin::Mutex;

pub trait KeyboardDevice: Send {
    fn read_scancode(&mut self) -> Option<u8>;
}

static KEYBOARD: Mutex<Option<&'static mut dyn KeyboardDevice>> =
    Mutex::new(None);

pub fn register(device: &'static mut dyn KeyboardDevice) {
    *KEYBOARD.lock() = Some(device);
}

pub fn read_scancode() -> Option<u8> {
    KEYBOARD
        .lock()
        .as_mut()
        .and_then(|k| k.read_scancode())
}

pub mod keyboard;