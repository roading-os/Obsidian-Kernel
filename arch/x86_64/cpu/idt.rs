// arch/x86_64/cpu/idt.rs
#![allow(dead_code)]
#![feature(abi_x86_interrupt)]

use core::arch::asm;
use x86_64::structures::idt::InterruptStackFrame;

// -------------------------------
// IDT entry structure
// -------------------------------
#[repr(C, packed)]
#[derive(Clone, Copy)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    type_attr: u8,
    offset_mid: u16,
    offset_high: u32,
    zero: u32,
}

impl IdtEntry {
    const fn missing() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist: 0,
            type_attr: 0,
            offset_mid: 0,
            offset_high: 0,
            zero: 0,
        }
    }

    /// Set the handler function for this IDT entry
    fn set_handler_addr(&mut self, addr: u64, user_callable: bool) {
    self.offset_low = addr as u16;
    self.offset_mid = (addr >> 16) as u16;
    self.offset_high = (addr >> 32) as u32;
    self.selector = 0x08;
    self.ist = 0;
    self.type_attr = if user_callable {
        0xEE // ring 3
    } else {
        0x8E // ring 0
    };
    self.zero = 0;
   }
}

#[repr(C, packed)]
struct IdtPointer {
    limit: u16,
    base: u64,
}

// -------------------------------
// IDT table (256 entries)
// -------------------------------
static mut IDT: [IdtEntry; 256] = [IdtEntry::missing(); 256];

// -------------------------------
// Default CPU exception handlers
// -------------------------------
extern "x86-interrupt" fn divide_by_zero(_stack: InterruptStackFrame) {
    panic!("EXCEPTION: DIVIDE BY ZERO");
}

extern "x86-interrupt" fn breakpoint(_stack: InterruptStackFrame) {
    // just return
}

// -------------------------------
// Syscall handler (interrupt 0x80)
// -------------------------------
extern "x86-interrupt" fn syscall_handler(_stack: InterruptStackFrame) {
    use crate::include::obsidian::{types::SyscallRegs, syscall::handle_syscall};

    // TODO: later we can actually read registers from userland
    let regs = SyscallRegs {
        rax: 0, rbx: 0, rcx: 0, rdx: 0,
        rsi: 0, rdi: 0,
        r8: 0, r9: 0, r10: 0, r11: 0,
        r12: 0, r13: 0, r14: 0, r15: 0,
    };

    handle_syscall(regs.rax, &regs);
}

// -------------------------------
// Initialize IDT and enable interrupts
// -------------------------------
pub fn init() {
    unsafe {
        // CPU exceptions
        IDT[0].set_handler_addr(divide_by_zero as u64); // #DE Divide-by-zero
        IDT[3].set_handler_addr(breakpoint as u64);     // #BP Breakpoint

        IDT[32].set_handler_addr(timer_interrupt as u64); // PIT

        // Syscall interrupt
        IDT[0x80].set_handler_addr(syscall_entry as u64); // interrupt 0x80 for syscalls

        // Load IDT
        let idt_ptr = IdtPointer {
            limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
            base: &IDT as *const _ as u64,
        };

        asm!("lidt [{}]", in(reg) &idt_ptr);
        asm!("sti"); // enable CPU interrupts
    }
}

extern "x86-interrupt" fn timer_interrupt(_stack: InterruptStackFrame) {
    
    unsafe {
        use x86_64::instructions::port::Port;
        let mut pic = Port::<u8>::new(0x20);
        pic.write(0x20); // EOI
    }

    crate::tasks::scheduler::schedule();
}

extern "C" {
    fn syscall_entry();
}