// arch/x86_64/memory/mod.rs
pub mod paging;
pub mod frame_alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::memory::map;
use crate::kernel::logger;
use crate::memory::map::MemoryRegionType;
use crate::arch::x86_64::memory::{frame_alloc, paging};

pub fn init(mb_addr: usize) {
    logger::info("Inicializando subsistema de memória");

    // Parser memory map of Multiboot2
    let mut regions = [map::MemoryRegion {
        start: 0,
        end: 0,
        region_type: map::MemoryRegionType::Reserved,
    }; 64]; // adjust if needs more regions

    let mut count = 0;
    unsafe {
        map::parse_memory_map(mb_addr, |region| {
            if count < regions.len() {
                regions[count] = region;
                count += 1;
            }
        });
    }
     
    let boxed_regions = Box::new(regions[..count].to_vec());
    let static_regions: &'static [_] = Box::leak(boxed_regions);
        
    let memory_map = map::MemoryMap {
        regions: static_regions,
    };

    logger::info("Mapa de memória parseado");

    // Initalize physic frame allocator
    frame_alloc::init(&memory_map);
     logger::info("Frame allocator inicializado");

    // Initializes paging (needed to map kernel + heap)
     logger::info("Paging inicializado");
}
