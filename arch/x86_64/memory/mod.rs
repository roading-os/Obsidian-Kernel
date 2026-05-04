// arch/x86_64/memory/mod.rs
pub mod paging;
pub mod frame_alloc;

use crate::kernel::logger;
use crate::memory::map;

pub fn init(mb_addr: usize) {
    logger::info("Inicializando subsistema de memoria");

    let memory_map = unsafe { map::from_multiboot(mb_addr) };

    logger::info("Mapa de memoria parseado");

    // Initialize physical frame allocator.
    frame_alloc::init(&memory_map);
    logger::info("Frame allocator inicializado");

    // Initialize paging (needed to map kernel + heap).
    paging::init();
    logger::info("Paging inicializado");
}
