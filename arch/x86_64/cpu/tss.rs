// arch/x86_64/cpu/tss.rs

use core::mem::size_of;

#[repr(C, packed)]
pub struct Tss {
    _reserved1: u32,
    pub rsp: [u64; 3],
    _reserved2: u64,
    pub ist: [u64; 7],
    _reserved3: u64,
    _reserved4: u16,
    pub iopb_offset: u16,
}

impl Tss {
    pub const fn new() -> Self {
        Self {
            _reserved1: 0,
            rsp: [0; 3],
            _reserved2: 0,
            ist: [0; 7],
            _reserved3: 0,
            _reserved4: 0,
            iopb_offset: size_of::<Tss>() as u16,
        }
    }
}

static mut TSS: Tss = Tss::new();

pub unsafe fn init(stack_top: u64) {
    let rsp0 = (core::ptr::addr_of_mut!(TSS) as *mut u8).add(4) as *mut u64;
    core::ptr::write_unaligned(rsp0, stack_top);
}

pub fn get_tss_ptr() -> u64 {
    core::ptr::addr_of!(TSS) as *const _ as u64
}
