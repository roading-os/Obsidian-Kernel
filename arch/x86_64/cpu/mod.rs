// arch/x86_64/cpu/mod.rs
pub mod gdt;
pub mod pic;
pub mod idt;
pub mod interrupts;

pub fn init() {
    gdt::init();
    pic::init();
    idt::init();
    interrupts::enable();
}
