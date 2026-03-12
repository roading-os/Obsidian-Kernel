use crate::tasks::task::{Task, TaskState};
use crate::drivers::timer::pit;

extern "C" {
    fn context_switch(old: *mut u64, new: *const u64);
}

static mut TASKS: [Option<Task>; 64] = [None; 64];
static mut CURRENT: usize = 0;

pub fn init() {
    let idle = create(idle_task);
    add(idle);
}

pub fn create(entry: fn()) -> Task {
    let stack = allocate_stack();

    let stack_top = stack + STACK_SIZE;

    let rsp = stack_top - 8;

    unsafe {
        *(rsp as *mut u64) = entry as u64;
    }

    Task {
        pid: alloc_pid(),
        rsp,
        state: TaskState::Ready,
    }
}

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

        // encontrar próxima task READY
        for _ in 0..TASKS.len() {
            CURRENT = (CURRENT + 1) % TASKS.len();

            if let Some(ref task) = TASKS[CURRENT] {
                if task.state == TaskState::Ready {
                    break;
                }
            }
        }

        if old == CURRENT {
            return;
        }

        let old_task = TASKS[old].as_mut().unwrap();
        let new_task = TASKS[CURRENT].as_ref().unwrap();

        old_task.state = TaskState::Ready;

        // troca de contexto
        context_switch(
            &mut old_task.rsp,
            &new_task.rsp
        );
    }
}