// drivers/video/mod.rs
pub trait VideoDevice {
    fn clear(&self, color: u32);
    fn put_pixel(&self, x: u32, y: u32, color: u32);
}

static mut VIDEO: Option<&'static dyn VideoDevice> = None;

pub fn register(device: &'static dyn VideoDevice) {
    unsafe {
        VIDEO = Some(device);
    }
}

pub fn clear(color: u32) {
    unsafe {
        if let Some(v) = VIDEO {
            v.clear(color);
        }
    }
}

pub fn put_pixel(x: u32, y: u32, color: u32) {
    unsafe {
        if let Some(v) = VIDEO {
            v.put_pixel(x, y, color);
        }
    }
}

pub mod framebuffer;
