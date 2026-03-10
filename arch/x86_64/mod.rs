// arch/x86_64/mod.rs
pub mod boot;
pub mod cpu;
pub mod memory;

pub fn init(mb_addr: usize) {
    cpu::init();
    memory::init(mb_addr);
}