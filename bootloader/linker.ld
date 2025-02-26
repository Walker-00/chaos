ENTRY(_start)

SECTIONS
{
    . = 1M;  /* Kernel loads at 1MB */

    .multiboot_header ALIGN(8) :
    {
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text)
    }

    .rodata :
    {
        *(.rodata)
    }

    .data :
    {
        *(.data)
    }

    .bss (NOLOAD) :
    {
        *(.bss)
    }
}

