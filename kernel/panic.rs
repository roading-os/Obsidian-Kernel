// kernel/panic.rs
use core::panic::PanicInfo;
use crate::drivers::serial;

pub fn panic(info: &PanicInfo) -> ! {
    serial::write_str("\nKERNEL PANIC\n");

    let msg = info.message();

    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}