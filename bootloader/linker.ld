ENTRY(_start)

SECTIONS
{
    . = 1M; /* Load the kernel at 1MB */

    .multiboot_header :
    {
        KEEP(*(.multiboot_header_func))
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

    /* Define kernel start and end */
    _kernel_start = 1M;
    _kernel_end = .;
}

