// kernel/mod.rs
pub mod init;
pub mod logger;
pub mod panic;
pub mod state;

pub fn init() {
    logger::init();
}