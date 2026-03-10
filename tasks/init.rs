// tasks/init.rs
pub fn init_task() {
    let task = Task {
        pid: alloc_pid(),
        context: Context {
            r15:0,r14:0,r13:0,r12:0,
            rbx:0,rbp:0,
            rip: idle_task as u64
        },
        stack_top: allocate_stack(),
        state: TaskState::Ready,
    };

    fn idle_task() -> ! {
        loop {
            unsafe {
                core::arch::asm!("hlt");
            }
        }       
    }

    scheduler::add(task);
}