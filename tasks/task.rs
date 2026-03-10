// tasks/task.rs
use crate::tasks::context::Context;
use crate::tasks::pid::Pid;

#[derive(Clone, Copy)]
pub struct Task {
    pub pid: Pid,
    pub context: Context,
    pub stack_top: u64,
    pub state: TaskState,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Sleeping,
    Dead,
}