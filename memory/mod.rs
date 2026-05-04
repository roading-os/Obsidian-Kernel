// memory/mod.rs

pub mod map;
pub mod heap;

use alloc::alloc::alloc as global_alloc;
use core::alloc::Layout;
use crate::arch::x86_64::memory::{frame_alloc, paging};
use crate::kernel::logger;

const HEAP_SIZE: usize = 1024 * 1024;

pub fn init(mb_addr: usize) {
    logger::info("Loading memory map");

    let memory_map = unsafe { map::from_multiboot(mb_addr) };

    frame_alloc::init(&memory_map);
    logger::info("Frame allocator ready");

    unsafe {
        paging::init();
    }

    logger::info("Paging initialized");

    let heap_start = paging::map_kernel_heap(HEAP_SIZE);

    unsafe {
        heap::init_heap(heap_start, HEAP_SIZE);
    }

    logger::info("Heap ready");
}

pub fn alloc(size: usize) -> Option<*mut u8> {
    let layout = Layout::from_size_align(size, 8).ok()?;
    let ptr = unsafe { global_alloc(layout) };

    if ptr.is_null() {
        None
    } else {
        Some(ptr)
    }
}

pub fn free(_ptr: *mut u8) {
    // The current syscall ABI does not pass the allocation size, so this
    // cannot safely reconstruct the Layout required for deallocation yet.
}

pub fn map(_virt: u64, _phys: u64, _size: u64) -> bool {
    false
}
