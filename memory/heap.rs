// memory/heap.rs
use core::alloc::Layout;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

/// Initializes the kernel heap
///
/// # Safety
/// - `heap_start` e `heap_size` need to point to a valid memory region for the heap
/// - This memory needs to be mapped and not used by anything else
pub unsafe fn init_heap(heap_start: usize, heap_size: usize) {
    HEAP.lock().init(heap_start as *mut u8, heap_size);
}

/// Fallback if trying to allocate before the heap exists
#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!(
        "Failed to allocate memory: size={}, alignment={}",
        layout.size(),
        layout.align()
    );
}

