// tasks/init.rs
use crate::tasks::scheduler;
use crate::tasks::scheduler::create;

pub fn idle_task() {
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

pub fn init() {
    let idle = create(idle_task);
    scheduler::add(idle);
}