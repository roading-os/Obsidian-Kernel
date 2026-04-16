// include/obsidian/syscall.rs
use crate::drivers::serial;
use crate::include::obsidian::{errno::*, types::*};

use crate::tasks;
use crate::memory;

//
// SYSCALL NUMBERS
//

pub const SYS_WRITE: u64 = 1;

pub const SYS_CREATE_PROCESS: u64 = 2;
pub const SYS_START_PROCESS: u64 = 3;
pub const SYS_KILL_PROCESS: u64 = 4;

pub const SYS_WAKE: u64 = 5;
pub const SYS_TIME: u64 = 6;
pub const SYS_SLEEP: u64 = 7;

pub const SYS_GET_PID: u64 = 8;

pub const SYS_MEMORY_ALLOC: u64 = 9;
pub const SYS_MEMORY_FREE: u64 = 10;
pub const SYS_MEMORY_MAP: u64 = 11;


//
// SYSCALL DISPATCHER
//

pub fn handle_syscall(number: u64, regs: &SyscallRegs) -> i64 {
    match number {

        SYS_WRITE => sys_write(regs),

        SYS_CREATE_PROCESS => sys_create_process(regs),

        SYS_START_PROCESS => sys_start_process(regs),

        SYS_KILL_PROCESS => sys_kill_process(regs),

        SYS_WAKE => sys_wake(regs),

        SYS_TIME => sys_time(),

        SYS_SLEEP => sys_sleep(regs),

        SYS_GET_PID => sys_get_pid(),

        SYS_MEMORY_ALLOC => sys_memory_alloc(regs),

        SYS_MEMORY_FREE => sys_memory_free(regs),

        SYS_MEMORY_MAP => sys_memory_map(regs),

        _ => -EINVAL as i64,
    }
}

//
// SYS_WRITE
//

fn sys_write(regs: &SyscallRegs) -> i64 {
    let ptr = regs.rdi as *const u8;
    let len = regs.rsi as usize;

    unsafe {
        for i in 0..len {
            serial::write_byte(*ptr.add(i));
        }
    }

    SUCCESS as i64
}

//
// PROCESS MANAGEMENT
//

fn sys_create_process(regs: &SyscallRegs) -> i64 {
   // placeholder until ELF loader
   -EINVAL as i64
}

fn sys_start_process(regs: &SyscallRegs) -> i64 {
    let pid = regs.rdi as u32;

    if tasks::start_process(pid) {
        SUCCESS as i64
    } else {
        -EINVAL as i64
    }
}

fn sys_kill_process(regs: &SyscallRegs) -> i64 {
    let pid = regs.rdi as u32;

    if tasks::kill_process(pid) {
        SUCCESS as i64
    } else {
        -EINVAL as i64
    }
}

//
// TASK CONTROL
//

fn sys_sleep(regs: &SyscallRegs) -> i64 {
    let time = regs.rdi;

    tasks::sleep(time);

    SUCCESS as i64
}

fn sys_wake(regs: &SyscallRegs) -> i64 {
    let pid = regs.rdi as u32;

    if tasks::wake(pid) {
        SUCCESS as i64
    } else {
        -EINVAL as i64
    }
}

fn sys_time() -> i64 {
    tasks::time() as i64
}

fn sys_get_pid() -> i64 {
    tasks::current_pid() as i64
}

//
// MEMORY
//

fn sys_memory_alloc(regs: &SyscallRegs) -> i64 {
    let size = regs.rdi as usize;

    match memory::alloc(size) {
        Some(ptr) => ptr as i64,
        None => -ENOMEM as i64,
    }
}

fn sys_memory_free(regs: &SyscallRegs) -> i64 {
    let ptr = regs.rdi as *mut u8;

    memory::free(ptr);

    SUCCESS as i64
}

fn sys_memory_map(regs: &SyscallRegs) -> i64 {
    let virt = regs.rdi;
    let phys = regs.rsi;
    let size = regs.rdx;

    if memory::map(virt, phys, size) {
        SUCCESS as i64
    } else {
        -EINVAL as i64
    }
}