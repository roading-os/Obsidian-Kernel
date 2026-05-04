// tasks/mod.rs
pub mod task;
pub mod thread;
pub mod scheduler;
pub mod context;
pub mod pid;
pub mod init;

pub fn init() {
    scheduler::init();
}

pub fn start_process(_pid: pid::Pid) -> bool {
    false
}

pub fn kill_process(_pid: pid::Pid) -> bool {
    false
}

pub fn sleep(_ticks: u64) {}

pub fn wake(_pid: pid::Pid) -> bool {
    false
}

pub fn time() -> u64 {
    0
}

pub fn current_pid() -> pid::Pid {
    0
}
