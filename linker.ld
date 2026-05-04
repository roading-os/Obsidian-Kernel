/*-------------------------------------------------------
  Linker script for Roading OS Kernel
  x86_64, Multiboot2
--------------------------------------------------------*/

/* Kernel entry point */
ENTRY(_start)

/* ------------------------------------------------------
   Kernel load address
   1 MB (0x100000) - standard for GRUB Multiboot
------------------------------------------------------ */
SECTIONS
{
    /* Start at 1 MB */
    . = 0x100000;

    /* --------------------------------------------------
       Multiboot2 header
       Must be early in the binary for GRUB to find it
    -------------------------------------------------- */
    .multiboot2 ALIGN(8) : {
        KEEP(*(.multiboot2))
        KEEP(*(.multiboot))
    }

    /* --------------------------------------------------
       Executable code
       Page-aligned (4 KB)
    -------------------------------------------------- */
    .text ALIGN(4K) : {
        *(.text*)
    }

    /* --------------------------------------------------
       Read-only data (constants)
       Page-aligned
    -------------------------------------------------- */
    .rodata ALIGN(4K) : {
        *(.rodata*)
    }

    /* --------------------------------------------------
       Initialized data
       Page-aligned
    -------------------------------------------------- */
    .data ALIGN(4K) : {
        *(.data*)
    }

    /* --------------------------------------------------
       Uninitialized data (BSS + COMMON)
       NOLOAD: does not take space in the binary
    -------------------------------------------------- */
    .bss ALIGN(4K) (NOLOAD) : {
        *(.bss*)
        *(COMMON)
    }
}
