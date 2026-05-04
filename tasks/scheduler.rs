// tasks/scheduler.rs
use crate::tasks::task::{Task, TaskState};
use crate::tasks::pid::alloc_pid;
use crate::tasks::init::idle_task;

extern "C" {
    fn context_switch(old: *mut u64, new: *const u64);
}

const STACK_SIZE: u64 = 4096;
const MAX_TASKS: usize = 64;

fn allocate_stack() -> u64 {
    use crate::arch::x86_64::memory::frame_alloc;

    let base = frame_alloc::alloc_frame().expect("no memory");

    base
}

static mut TASKS: [Option<Task>; MAX_TASKS] = [None; MAX_TASKS];
static mut CURRENT: usize = 0;

pub fn init() {
    let idle = create(idle_task);
    add(idle);
}

pub fn create(entry: fn()) -> Task {
    let stack = allocate_stack();

    let stack_top = stack + STACK_SIZE;
    let rsp = stack_top - 7 * 8;

    unsafe {
        let frame = rsp as *mut u64;
        // context_switch pops six callee-saved registers and then returns.
        for i in 0..6 {
            *frame.add(i) = 0;
        }
        *frame.add(6) = entry as u64;
    }

    Task {
        pid: alloc_pid(),
        rsp,
        state: TaskState::Ready,
    }
}

pub fn add(task: Task) {
    unsafe {
        let mut i = 0;
        while i < MAX_TASKS {
            if TASKS[i].is_none() {
                TASKS[i] = Some(task);
                return;
            }

            i += 1;
        }
    }
}

pub fn schedule() {
    unsafe {
        let old = CURRENT;

        // encontrar próxima task READY
        for _ in 0..MAX_TASKS {
            CURRENT = (CURRENT + 1) % MAX_TASKS;

            if let Some(ref task) = TASKS[CURRENT] {
                if task.state == TaskState::Ready {
                    break;
                }
            }
        }

        if old == CURRENT {
            return;
        }

        let old_rsp = match TASKS[old].as_mut() {
            Some(t) => {
                t.state = TaskState::Ready;
                &mut t.rsp as *mut u64
            }
            None => return,
        };

        let new_rsp = match TASKS[CURRENT].as_mut() {
            Some(t) => {
                t.state = TaskState::Running;
                &t.rsp as *const u64
            }
            None => return,
        };

        // troca de contexto
        context_switch(old_rsp, new_rsp);
    }
}
