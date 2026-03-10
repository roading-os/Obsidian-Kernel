// memory/mod.rs

pub mod map;
pub mod heap;

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