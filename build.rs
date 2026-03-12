// build.rs
fn main() {
    cc::Build::new()
        .files(&["arch/x86_64/boot/multiboot.S",
                 "arch/x86_64/boot/long_mode.S",
                 "arch/x86_64/cpu/context_switch.S",
                 "arch/x86_64/cpu/syscall_entry.S",
                 ])
        .flag("-m64")
        .compile("multiboot");
}        
