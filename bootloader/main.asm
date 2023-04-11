global start ; start entry header
extern long_mode_start ; long mode

section .text ; text section
bits 32 ; 32 bits for bootloading
start: ; start entry point
	mov esp, stack_top ; move stack_top to esp register

	call check_multiboot ; call multiboot checker func
	call check_cpuid ; call cpuid checker func
	call check_long_mode ; call long mode support checker func

	call setup_page_tables ; call page tabels setup func
	call enable_paging ; call paging enable fun

	lgdt [gdt64.pointer] ; load the global descriptor table
	jmp gdt64.code_segment:long_mode_start ; jump to 64bit assembly

	hlt

check_multiboot: ; multiboot checker func
	cmp eax, 0x36d76289 ; store multiboot magic number to eax register
	jne .no_multiboot ; jump to no_multiboot func when multiboost isn't support
	ret ; if multiboot support return
.no_multiboot: ; func for no multiboot support
	mov al, "M" ; Error code M for multiboot error
	jmp error ; jump to the error handler func

check_cpuid: ; cpu id support checker
	pushfd ; push flag register to stack
	pop eax ; pop the stack to eax register
	mov ecx, eax ; copy the flag to ecx
	xor eax, 1 << 21 ; flip the id bit
	push eax ; push back to stack 
	popfd ; pop the flag register
	pushfd ; push the flag register
	pop eax ; copy back to eax register
	push ecx ; push ecx value
	popfd ; get original flag from ecx
	cmp eax, ecx ; compare eax and ecx
	je .no_cpuid ; jump to no_cpuid func when eax and ecx is match cuz not support
	ret ; if it's not match return cuz support
.no_cpuid: ; no_cpuid func for cpu not support
	mov al, "C" ; Error code C for cpu not support
	jmp error ; jump to the error func

check_long_mode: ; long mode 64bits checker func
	mov eax, 0x80000000 ; store magic number to eax register
	cpuid ; call cpuid inst which will get eax as arg and return the number which is greater than that magic number
	cmp eax, 0x80000001 ; compare the eax which original eax valu + 1
	jb .no_long_mode ; if eax is less than original eax valu + 1 jump to no_long_mode func cuz Long mode 64bits is not supported

	mov eax, 0x80000001 ; elase put 0x80000001 to eax
	cpuid ; call cpuid again but this time cpuid will store value in edx register
	test edx, 1 << 29 ; check the lm bit
	jz .no_long_mode ; if it's not lm bit jump to no_long_mode
	
	ret ; else return
.no_long_mode: ; No Long Mode func
	mov al, "L" ; L for No Long Mode Error Code
	jmp error ; Jump to the error func

setup_page_tables: ; func for setup page tables
	mov eax, page_table_l3 ; move level 3 page table's address to eax
	or eax, 0b11 ; present, writable flag
	mov [page_table_l4], eax ; take the address of eax which is added flag and move to level 4 page table 
	
	mov eax, page_table_l2 ; like up but level 2
	or eax, 0b11 ; present, writable flag
	mov [page_table_l3], eax ; like up but level 3

	mov ecx, 0 ; counter
.loop: ; loop
	mov eax, 0x200000 ; Map 2MiB memory
	mul ecx ; multiply the eax with ecx
	or eax, 0b10000011 ; present, writable, huge page flag
	mov [page_table_l2 + ecx * 8], eax ; move eax to level 2 table + ecx * 8

	inc ecx ; increment counter
	cmp ecx, 512 ; checks if the whole table is mapped
	jne .loop ; if not, continue

	ret ; return

enable_paging: ; pagging enable func
	; pass page table location to cpu
	mov eax, page_table_l4 ; move level 4 table's address to eax
	mov cr3, eax ; copy eax to cr3

	; enable Physical Address Extension(PAE)
	mov eax, cr4 ; copy cr4 to eax
	or eax, 1 << 5 ; enable 5 bit for PAE flag
	mov cr4, eax ; move eax to cr4

	; enable x86_64 64bits long mode
	mov ecx, 0xC0000080 ; put magic value to ecx
	rdmsr ; read model specific register
	or eax, 1 << 8 ; enable long mode flag
	wrmsr ; write back to msr(model specific register)

	; enable paging
	mov eax, cr0 ; copy pagging flag to eax
	or eax, 1 << 31 ; enable pagging bit(31)
	mov cr0, eax ; move back to cr0

	ret ; return

error: ; error handler func
	; print "ERR: X" where X is the error code
	mov dword [0xb8000], 0x4f524f45 ; ER
	mov dword [0xb8004], 0x4f3a4f52 ; RR
	mov dword [0xb8008], 0x4f204f20 ; :
	mov byte  [0xb800a], al ; Error code
	hlt ; hlt cpu

section .bss ; bss section
align 4096 ; align all table to 4kb
page_table_l4: ; level 4 page table
	resb 4096 ; 4kb
page_table_l3: ; level 3 page table
	resb 4096 ; 4kb
page_table_l2: ; level 2 page table
	resb 4096 ; 4kb
stack_bottom: ; stack bottom
	resb 4096 * 4 ; resb 16kb memory
stack_top: ; stack top

section .rodata ; rodata(read only data) section
gdt64: ; 64 bit global descriptor table
	dq 0 ; begin with zero entry
.code_segment: equ $ - gdt64 ; code_segment lable
	dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53) ; code segment
.pointer: ; pointer to gdt
	dw $ - gdt64 - 1 ; length of the table
	dq gdt64 ; address of the table
