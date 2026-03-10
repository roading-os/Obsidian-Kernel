// tasks/pid.rs
use core::sync::atomic::{AtomicU32, Ordering};

pub type Pid = u32;

static NEXT_PID: AtomicU32 = AtomicU32::new(1);

pub fn alloc_pid() -> Pid {
    NEXT_PID.fetch_add(1, Ordering::SeqCst)
}