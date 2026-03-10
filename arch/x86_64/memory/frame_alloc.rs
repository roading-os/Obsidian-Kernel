// arch/x86_64/memory/frame_alloc.rs
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::memory::map::MemoryRegionType;
use crate::memory::map::MemoryMap;

const FRAME_SIZE: u64 = 4096;

// Bitmap simples: 1 bit por frame
static mut BITMAP: [u8; 1024 * 1024] = [0; 1024 * 1024]; 
// → suporta até ~32 GiB (1M bytes * 8 bits * 4KiB)

static TOTAL_FRAMES: AtomicUsize = AtomicUsize::new(0);

#[inline(always)]
fn set_frame_used(frame: usize) {
    unsafe {
        BITMAP[frame / 8] |= 1 << (frame % 8);
    }
}

#[inline(always)]
fn set_frame_free(frame: usize) {
    unsafe {
        BITMAP[frame / 8] &= !(1 << (frame % 8));
    }
}

#[inline(always)]
fn is_frame_free(frame: usize) -> bool {
    unsafe {
        (BITMAP[frame / 8] & (1 << (frame % 8))) == 0
    }
}

pub fn init(memory_map: &MemoryMap) {
    // 1️⃣ Marca tudo como usado
    let max_frames = unsafe { BITMAP.len() * 8};
    TOTAL_FRAMES.store(max_frames, Ordering::Relaxed);

    for i in 0..max_frames {
        set_frame_used(i);
    }

    // 2️⃣ Libera apenas regiões Usable
    for region in memory_map.regions {
        reserve_region(0x0, 0x200000)
        if region.region_type != MemoryRegionType::Usable {
            continue;
        }

        let start_frame = region.start / FRAME_SIZE as usize;
        let end_frame = region.end / FRAME_SIZE as usize;

        for frame in start_frame..end_frame {
            if frame < max_frames {
                set_frame_free(frame);
            }
        }
    }
}

pub fn alloc_frame() -> Option<u64> {
    let max_frames = TOTAL_FRAMES.load(Ordering::Relaxed);

    for frame in 0..max_frames {
        if is_frame_free(frame) {
            set_frame_used(frame);
            return Some(frame as u64 * FRAME_SIZE);
        }
    }

    None
}
