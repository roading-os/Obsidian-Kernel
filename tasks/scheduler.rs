// tasks/scheduler.rs
use crate::tasks::task::{Task, TaskState};

extern "C" {
    fn context_switch(old: *mut u64, new: *const u64);
}

static mut TASKS: [Option<Task>; 64] = [None; 64];
static mut CURRENT: usize = 0;

pub fn init() {}

pub fn add(task: Task) {
    unsafe {
        for slot in TASKS.iter_mut() {
            if slot.is_none() {
                *slot = Some(task);
                return;
            }
        }
    }
}

pub fn schedule() {
    unsafe {
        let old = CURRENT;

        for _ in 0..TASKS.len() {
            CURRENT = (CURRENT + 1) % TASKS.len();

            if let Some(ref task) = TASKS[CURRENT] {
                if task.state == TaskState::Ready {
                    break;
                }
            }
        }

        if let (Some(ref mut old_task), Some(ref new_task)) =
            (&mut TASKS[old], &TASKS[CURRENT])
        {
            context_switch(
                &mut old_task.stack_top,
                &new_task.stack_top
            );
        }
    }
}