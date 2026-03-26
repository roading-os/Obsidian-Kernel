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
stack64:
    .skip 16384           # 16 KiB de stack

.section .text
long_mode_start:
    cli

    # Limpar BSS (PML4, PDPT, PD)
    mov $pml4, %rdi
    mov $4096*3, %rcx
    xor %rax, %rax
    rep stosb

    # Configurar PML4 → PDPT
    lea pdpt(%rip), %rax
    orq $0x3, %rax          # Presente + R/W
    movq %rax, pml4(%rip)

    # Configurar PDPT → PD
    lea pd(%rip), %rax
    orq $0x3, %rax
    movq %rax, pdpt(%rip)

    # Configurar PDs para mapear 0..4 GiB usando 2 MiB pages
    mov $0, %rcx
1:
    mov %rcx, %rax
    shl $21, %rax
    orq $0x83, %rax          # Presente + R/W + Page Size
    movq %rax, pd(,%rcx,8)
    inc %rcx
    cmp $2048, %rcx          # 2048 * 2 MiB = 4 GiB
    jne 1b

    # Ativar CR3
    lea pml4(%rip), %rax
    mov %rax, %cr3

    # Habilitar PAE (CR4.PAE)
    mov %cr4, %rax
    orq $0x20, %rax
    mov %rax, %cr4

    # Habilitar Long Mode via MSR
    mov $0xC0000080, %ecx    # IA32_EFER
    rdmsr
    orl $0x100, %eax          # LME
    xor %edx, %edx
    wrmsr

    # Ativar paging (CR0.PG)
    mov %cr0, %rax
    orq $0x80000000, %rax
    mov %rax, %cr0

    # Inicializar stack 64-bit
    lea stack64+16384(%rip), %rsp

    # Configurar GDT 64-bit
    lgdt gdt64_ptr(%rip)

    # Saltar para kernel 64-bit
    pushq $0x08                # selector de código 64-bit
    lea kernel_main_64(%rip), %rax
    push %rax
    lretq

.section .rodata
gdt64:
    .quad 0x0000000000000000  # Null
    .quad 0x00AF9A0000000000  # Código 64-bit: exec, R/W
    .quad 0x00AF920000000000  # Dados 64-bit: R/W

gdt64_ptr:
    .word 3*8 - 1              # tamanho GDT
    .quad gdt64