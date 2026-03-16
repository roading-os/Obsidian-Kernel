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