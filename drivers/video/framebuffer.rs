// drivers/video/framebuffer.rs
use core::ptr::write_volatile;
use super::VideoDevice;

#[repr(C)]
pub struct FramebufferInfo {
    pub addr: u64,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
}

pub struct Framebuffer {
    info: &'static FramebufferInfo,
}

impl Framebuffer {
    pub fn new(info: &'static FramebufferInfo) -> Self {
        assert!(info.bpp == 32);
        Self { info }
    }
}

impl VideoDevice for Framebuffer {
    fn clear(&self, color: u32) {
        for y in 0..self.info.height {
            for x in 0..self.info.width {
                self.put_pixel(x, y, color);
            }
        }
    }

    fn put_pixel(&self, x: u32, y: u32, color: u32) {
        let offset = (y * self.info.pitch + x * 4) as usize;
        unsafe {
            write_volatile(
                (self.info.addr as *mut u8).add(offset) as *mut u32,
                color,
            );
        }
    }
}
