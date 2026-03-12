// arch/x86_64/memory/paging.rs
use crate::arch::x86_64::memory::frame_alloc;
use core::arch::asm;

const PAGE_SIZE: u64 = 4096;
const ENTRY_COUNT: usize = 512;

const PRESENT: u64 = 1 << 0;
const WRITABLE: u64 = 1 << 1;
const USER: u64 = 1 << 2;

static mut PML4: *mut u64 = 0 as *mut u64;

pub fn init() {
    unsafe {
        PML4 = create_pml4();

        // Identity map do kernel (0x100000 .. 0x2000000 por exemplo)
        map_range(0x100000, 0x100000, 16 * 1024 * 1024, PRESENT | WRITABLE);

        // Higher half kernel
        map_range(0xffffffff80000000, 0x100000, 16 * 1024 * 1024, PRESENT | WRITABLE);

        // Enable paging
        enable_paging(PML4);
    }
}

// Cria PML4 vazio
unsafe fn create_pml4() -> *mut u64 {
    let frame = frame_alloc::alloc_frame().expect("No free frame for PML4") as *mut u64;
    for i in 0..ENTRY_COUNT {
        *frame.add(i) = 0;
    }
    frame
}

// Mapear range de memória (4KiB páginas)
unsafe fn map_range(virt_start: u64, phys_start: u64, size: u64, flags: u64) {
    let mut offset = 0;
    while offset < size {
        let va = virt_start + offset;
        let pa = phys_start + offset;

        map_page(va, pa, flags);
        offset += PAGE_SIZE;
    }
}

// Mapear página única (4KiB)
unsafe fn map_page(virt: u64, phys: u64, flags: u64) {
    let pml4 = PML4;

    let pml4_i = ((virt >> 39) & 0x1FF) as usize;
    let pdpt_i = ((virt >> 30) & 0x1FF) as usize;
    let pd_i   = ((virt >> 21) & 0x1FF) as usize;
    let pt_i   = ((virt >> 12) & 0x1FF) as usize;

    let pdpt = get_or_create(pml4, pml4_i);
    let pd   = get_or_create(pdpt, pdpt_i);
    let pt   = get_or_create(pd, pd_i);

    *pt.add(pt_i) = phys | flags | PRESENT;
}

unsafe fn get_or_create(table: *mut u64, index: usize) -> *mut u64 {
    let entry = table.add(index);

    if *entry & PRESENT == 0 {
        let frame = frame_alloc::alloc_frame().unwrap();
        let new_table = frame as *mut u64;

        for i in 0..ENTRY_COUNT {
            *new_table.add(i) = 0;
        }

        *entry = frame | PRESENT | WRITABLE;
        new_table
    } else {
        (*entry & 0x000fffff_fffff000) as *mut u64
    }
}

// Ativar paging (CR3)
unsafe fn enable_paging(pml4_addr: *mut u64) {
    asm!(
        "mov cr3, {0}",
        in(reg) pml4_addr,
        options(nostack, preserves_flags)
    );
}

// Mapeia heap do kernel e retorna endereço virtual inicial
pub fn map_kernel_heap(size: usize) -> usize {
    const HEAP_START: u64 = 0xffff_ffff_9000_0000;

    unsafe {
      for offset in (0..size).step_by(PAGE_SIZE) {
         let frame = frame_alloc::alloc_frame().unwrap();
         map_page(HEAP_START + offset, frame, PRESENT | WRITABLE);
      }
        
    }

    HEAP_START as usize
}
