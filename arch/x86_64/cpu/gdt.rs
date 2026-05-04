// arch/x86_64/cpu/gdt.rs
use core::arch::asm;

#[repr(C, packed)]
struct GdtDescriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtDescriptor {
    const fn new(base: u32, limit: u32, access: u8, flags: u8) -> Self {
        Self {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_mid: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: (((limit >> 16) & 0x0F) as u8) | (flags & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,

        }
    }
}

#[repr(C, packed)]
struct TssDescriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
    base_upper: u32,
    reserved: u32,
}

#[repr(C, packed)]
struct GdtPointer {
    limit: u16,
    base: u64,
}

static GDT: [GdtDescriptor; 3] = [
    // Null descriptor
    GdtDescriptor::new(0, 0, 0, 0),

    // Kernel code segment
    GdtDescriptor::new(
        0,
        0xFFFFF,
        0x9A, // present, ring 0, executable, readable
        0xA0, // granularity=1, long mode=1
    ),

    // Kernel data segment
    GdtDescriptor::new(
        0,
        0xFFFFF,
        0x92, // present, ring 0, writable
        0xA0, // granularity=1
    ),    
];

pub fn init() {
    let gdt_ptr = GdtPointer {
        limit: (core::mem::size_of::<[GdtDescriptor; 3]>() - 1) as u16,
        base: &GDT as *const _ as u64,
    };

    unsafe {
        // Load GDT
        asm!("lgdt [{}]", in(reg) &gdt_ptr);

        // Reload segment registers
        asm!(
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            in("ax") 0x10u16,
            options(nostack, preserves_flags),
        );
    }
}

static mut KERNEL_STACK: [u8; 4096 * 4] = [0; 4096 * 4];

fn kernel_stack_top() -> u64 {
    core::ptr::addr_of!(KERNEL_STACK) as *const _ as u64 + (4096 * 4) as u64
}

fn create_tss_descriptor(base: u64, limit: u32) -> TssDescriptor {
    TssDescriptor {
        limit_low: (limit & 0xFFFF) as u16,
        base_low: (base & 0xFFFF) as u16,
        base_mid: ((base >> 16) & 0xFF) as u8,
        access: 0x89, // present + type TSS
        granularity: ((limit >> 16) & 0x0F) as u8,
        base_high: ((base >> 24) & 0xFF) as u8,
        base_upper: (base >> 32) as u32,
        reserved: 0,
    }
}
