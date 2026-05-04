# arch/x86_64/boot/multiboot.S
/*-------------------------------------------------------
 Multiboot2 + x86_64 entry stub
 Compatible with modular Obsidian Kernel (framebuffer, keyboard, serial)
 AT&T syntax (GAS)
--------------------------------------------------------*/

.section .multiboot2, "a"
.align 8

/*-----------------------------
  Multiboot2 Header
-----------------------------*/
multiboot_header:
.long 0xe85250d6                 /* Multiboot2 magic */
.long 0                          /* Architecture (0 = i386, required even for x86_64) */
.long header_end - multiboot_header  /* Total header length */
.long -(0xe85250d6 + 0 + (header_end - multiboot_header)) /* Checksum */

header_start:

/* Framebuffer REQUEST TAG (type = 5) */
.short 5        /* tag type: framebuffer request */
.short 0        /* flags */
.long 24       /* tag size */
.long 1024      /* preferred width */
.long 768       /* preferred height */
.long 32        /* preferred bits per pixel */
.long 0

/* END TAG */
.short 0
.short 0
.long 8

header_end:

/*-----------------------------
 Kernel entry point (64-bit)
-----------------------------*/
.section .text
.global _start
.extern long_mode_start

_start:
    /* --------------------------
       Initialize 64-bit stack
    -------------------------- */
    lea stack_top(%rip), %rsp

    /* --------------------------
       Clear .bss section manually
    -------------------------- */
    mov $bss_start, %rdi
    mov $bss_end, %rcx
    xor %rax, %rax
.clear_bss:
    cmp %rdi, %rcx
    jge .bss_done
    movq %rax, (%rdi)
    add $8, %rdi
    jmp .clear_bss
.bss_done:

    /* --------------------------
       Save Multiboot info address
       (passed in EBX by GRUB in 32-bit mode)
    -------------------------- */
    mov %ebx, %edi        /* zero-extend EBX to RDI automatically */
                          /* %edi is 32-bit part of %rdi; upper 32 bits zeroed */

    /* --------------------------
       Jump to 64-bit long mode
       long_mode_start: label in long_mode.S
    -------------------------- */
    call long_mode_start

.hang:
    hlt
    jmp .hang

/*-----------------------------
 Kernel stack (aligned)
-----------------------------*/
.section .bss
.align 16
stack:
    .skip 16384
stack_top:

/*-----------------------------
 BSS start/end symbols
-----------------------------*/
.global bss_start
.global bss_end

bss_start:
    # Here starts the real BSS of kernel, without resetting
    . = .  # Not effect, just guarantee that symbol exists

bss_end:
    # Here ends BSS
