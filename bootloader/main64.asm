global long_mode_start ; global long_mode_start
section .text ; text section
bits 64 ; 64 bits assembly
long_mode_start: ; long mode start lable
    ; load null into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

extern kernel_start ; call kernel_main
call kernel_start ; call the kernel_main func
    hlt ; halt the cpu
