# arch/x86_64/boot/long_mode.S
.global long_mode_start
.extern kernel_main_64

.section .bss
.align 4096
pml4:
    .skip 4096
pdpt:
    .skip 4096
pd:
    .skip 4096

.section .text
long_mode_start:
    cli

    # Limpar BSS
    mov $pml4, %rdi
    mov $4096*3, %rcx
    xor %rax, %rax
    rep stosb

    # Configurar PML4
    lea pdpt(%rip), %rax
    orq $0x3, %rax
    movq %rax, pml4(%rip)

    # Configurar PDPT
    lea pd(%rip), %rax
    orq $0x3, %rax
    movq %rax, pdpt(%rip)

    # Configurar PDs
    mov $0, %rcx
1:
    mov %rcx, %rax
    shl $21, %rax
    orq $0x83, %rax          # 64-bit
    movq %rax, pd(,%rcx,8)
    inc %rcx
    cmp $512, %rcx
    jne 1b

    # Ativar CR3
    lea pml4(%rip), %rax
    mov %rax, %cr3

    # Habilitar PAE (CR4.PAE)
    mov %cr4, %rax
    orq $0x20, %rax
    mov %rax, %cr4

    # Habilitar Long Mode via MSR
    mov $0xC0000080, %ecx
    rdmsr
    orl $0x100, %eax        # 32-bit
    wrmsr

    # Ativar paging (CR0.PG)
    mov %cr0, %rax
    movabs $0x80000000, %rbx
    or %rbx, %rax
    mov %rax, %cr0

    # Saltar para kernel 64-bit
    lgdt gdt64_ptr(%rip)
    pushq $0x08
    lea kernel_main_64(%rip), %rax
    push %rax
    lretq

.section .rodata
gdt64:
    .quad 0
    .quad 0x00AF9A000000FFFF
    .quad 0x00AF92000000FFFF

gdt64_ptr:
    .word (3*8 - 1)
    .quad gdt64