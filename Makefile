kernel := kernel.bin
iso := chaos.iso
build_type := debug
rust_os := target/target/$(build_type)/libchaos.a
rust_toolchain := nightly

linker_scp := bootloader/linker.ld
grub_cfg := bootloader/grub.cfg
asm_src_files := $(wildcard bootloader/*.asm)
asm_obj_files := bootloader/*.o

setup:
	sudo apt install nasm
	rustup update $(rust_toolchain)
	rustup default $(rust_toolchain)
	rustup target add x86_64-unknown-none
	rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

build:
	cargo build
	nasm -f elf64 bootloader/header.asm -o bootloader/header.o
	nasm -f elf64 bootloader/main.asm -o bootloader/main.o
	nasm -f elf64 bootloader/main64.asm -o bootloader/main64.o
	ld -n --gc-sections -T $(linker_scp) -o $(kernel) $(asm_obj_files) $(rust_os)
	/bin/mkdir -p build/iso/boot/grub
	cp $(kernel) build/iso/boot/$(kernel)
	cp $(grub_cfg) build/iso/boot/grub/grub.cfg
	grub-mkrescue -o $(iso) build/iso 2> /dev/null
	rm -rf build/iso

run:
	qemu-system-x86_64 -cdrom $(iso)

clean:
	rm -rf build kernel.bin chaos.iso build bootloader/*.o
	cargo clean

