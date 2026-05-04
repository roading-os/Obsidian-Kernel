// src/main.rs
/* don't worry, inside src/ have symlinks of arch/, drivers/, include/,
kernel/ and memory/ */
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[path = "../arch/mod.rs"]
mod arch;
#[path = "../drivers/mod.rs"]
mod drivers;
#[path = "../kernel/mod.rs"]
mod kernel;
#[path = "../memory/mod.rs"]
mod memory;
#[path = "../include/mod.rs"]
mod include;
#[path = "../tasks/mod.rs"]
mod tasks;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn kernel_main_64(mb_addr: usize) -> ! {
    kernel::init::init(mb_addr)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::panic::panic(info)
}
