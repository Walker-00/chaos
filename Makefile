LD := ld
RUSTC := cargo
OBJCOPY := objcopy
LDFLAGS := -nostdlib
BOOTLOADER_DIR := bootloader
KERNEL_DIR := src

# Set output file name and format
OUTPUT := kernel.bin
OUTPUT_FORMAT := binary

all: $(OUTPUT)

.PHONY: idk
$(OUTPUT): $(BOOTLOADER_DIR)/main.asm $(BOOTLOADER_DIR)/header.asm $(BOOTLOADER_DIR)/main64.asm $(KERNEL_DIR)/lib.rs
	nasm -f elf64 $(BOOTLOADER_DIR)/main.asm -o $(BOOTLOADER_DIR)/main.o
	nasm -f elf64 $(BOOTLOADER_DIR)/header.asm -o $(BOOTLOADER_DIR)/header.o
	nasm -f elf64 $(BOOTLOADER_DIR)/main64.asm -o $(BOOTLOADER_DIR)/main64.o
	$(RUSTC) build --release
	objcopy -I binary -O elf64-x86-64 --binary-architecture i386:x86-64 target/target/release/chaos kernel.o
	x86_64-elf-ld -n -T $(BOOTLOADER_DIR)/linker.ld $(LDFLAGS) ./kernel.o $(BOOTLOADER_DIR)/main.o $(BOOTLOADER_DIR)/header.o $(BOOTLOADER_DIR)/main64.o -o kernel.bin
