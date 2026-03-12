// kernel/syscall.rs
use crate::include::obsidian::types::SyscallRegs;
use crate::include::obsidian::syscall::handle_syscall;

#[no_mangle]
pub extern "C" fn syscall_dispatch(regs: &mut SyscallRegs) -> u64 {
    handle_syscall(regs.rax, regs) as u64
}