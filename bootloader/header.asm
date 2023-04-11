section .multiboot_header ; multiboot_header header
header_start:
	dd 0xe85250d6 ; multiboot2 magic number
	; architecture
	dd 0 ; i386 32 bits protected mode
	; header length
	dd header_end - header_start
	; checksum
	dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

	; end tag
	dw 0
	dw 0
	dd 8
header_end: