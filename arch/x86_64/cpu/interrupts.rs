// arch/x86_64/cpu/interrupts.rs
use core::arch::asm;

/// Enable CPU interrupts (STI)
#[inline(always)]
pub fn enable() {
    unsafe {
        asm!("sti", options(nomem, nostack, preserves_flags));
    }
}

/// Disable CPU interrupts (CLI)
#[inline(always)]
pub fn disable() {
    unsafe {
        asm!("cli", options(nomem, nostack, preserves_flags));
    }
}
