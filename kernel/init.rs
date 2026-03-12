// kernel/init.rs
use crate::drivers::serial;
use crate::drivers::timer::pit;
use crate::kernel::logger;
use crate::memory;
use crate::tasks;
use crate::arch;
use core::sync::atomic::Ordering;

pub fn init(mb_addr: usize) -> ! {
    
    serial::init();
    logger::init();

    logger::info("Obsidian Kernel");
    logger::info("Entering 64-bit long mode");
     
    memory::init(mb_addr);

    arch::init(mb_addr);

    pit::init();

    tasks::init();
 
    logger::info("Memory initialized");
    logger::info("Kernel ready");

    kernel_loop()
}

fn kernel_loop() -> ! {
    loop {
        if !PROGRAM_RUNNING.load(Ordering::SeqCst) {
            idle();
        }

        unsafe { core::arch::asm!("hlt"); }
    }
}

fn idle() -> ! {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}