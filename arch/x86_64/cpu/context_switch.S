/* arch/x86_64/cpu/context_switch.S */
.global context_switch

context_switch:
    # rdi = &old_rsp
    # rsi = &new_rsp

    # save callee-saved registers
    push %rbp
    push %rbx
    push %r12
    push %r13
    push %r14
    push %r15
    
    mov %rsp, (%rdi) # save actual stack
    
    mov (%rsi), %rsp # load new stack
    
    # restore registers
    pop %r15
    pop %r14
    pop %r13
    pop %r12
    pop %rbx
    pop %rbp

    ret