// kernel/init.rs
use crate::drivers::serial;
use crate::drivers::timer::pit;
use crate::kernel::logger;
use crate::memory;
use crate::tasks;
use crate::arch;

pub fn init(mb_addr: usize) -> ! {
    
    serial::init();
    logger::init();

    logger::info("Obsidian Kernel");
    logger::info("Entering 64-bit long mode");
     
    memory::init(mb_addr);

    arch::x86_64::cpu::init();

    pit::init();

    tasks::init();
 
    logger::info("Memory initialized");
    logger::info("Kernel ready");

    kernel_loop()
}

fn kernel_loop() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

fn idle() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}