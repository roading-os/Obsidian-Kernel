// kernel/logger.rs
use crate::drivers::serial;

pub fn init() {}

pub fn info(msg: &str) {
    serial::write_str("[INFO] ");
    serial::write_str(msg);
    serial::write_str("\n");
}

pub fn error(msg: &str) {
    serial::write_str("[ERROR] ");
    serial::write_str(msg);
    serial::write_str("\n");
}