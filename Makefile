kernel := kernel.bin
iso := chaos.iso
build_type := debug
rust_os := target/target/$(build_type)/libchaos.a

linker_scp := bootloader/linker.ld
grub_cfg := bootloader/grub.cfg
asm_src_files := $(wildcard bootloader/*.asm)
asm_obj_files := $(wildcard bootloader/*.o)

build:
	cargo build
	nasm -f elf64 bootloader/main.asm -o bootloader/main.o
	nasm -f elf64 bootloader/main64.asm -o bootloader/main64.o
	nasm -f elf64 bootloader/header.asm -o bootloader/header.o
	ld -n --gc-sections -T $(linker_scp) -o $(kernel) $(asm_obj_files) $(rust_os)
	mkdir -p build/iso/boot/grub
	cp $(kernel) build/iso/boot/$(kernel)
	cp $(grub_cfg) build/iso/boot/grub/grub.cfg
	grub-mkrescue -o $(iso) build/iso 2> /dev/null
	rm -rf build/iso

run:
	qemu-system-x86_64 -cdrom $(iso)


