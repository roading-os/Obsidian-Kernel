// arch/x86_64/boot/mod.rs
extern "C" {
    fn long_mode_start();
    fn gdt64();
    fn gdt64_ptr();
    fn multiboot();
}