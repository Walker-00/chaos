alias s := setup
alias r := run
alias b := build
alias d := debug

# Kernel and ISO file names
kernel              := 'kernel.bin'
iso                 := 'chaos.iso'

# Build type and Rust OS target path
build_type          := 'debug'
rust_os             := 'target/target/'+ build_type + '/libchaos.a'

# Package manager settings
pkg_manager         := 'pacman'
pkg_manager_option  := '-S'

# Linker script and GRUB configuration
linker_scp          := 'bootloader/linker.ld'
grub_cfg            := 'bootloader/grub.cfg'

# Assembly source and object files
#asm_src_files       := 'bootloader/*.asm'
asm_obj_files       := 'bootloader/*.o'

# Setup task: Install necessary dependencies and configure Rust
setup:
    sudo  {{pkg_manager}} {{pkg_manager_option}} nasm xorriso
    rustup target add x86_64-unknown-none
    rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu

# Build task: Compile the Rust code and assemble bootloader files
#ld -n --gc-sections -T {{linker_scp}} -o {{kernel}} {{asm_obj_files}} {{rust_os}}
build:
    cargo build
    nasm -f elf64 bootloader/header.asm -o bootloader/header.o
    nasm -f elf64 bootloader/main.asm -o bootloader/main.o
    nasm -f elf64 bootloader/main64.asm -o bootloader/main64.o
    ld -n --gc-sections -T {{linker_scp}} -o {{kernel}} {{asm_obj_files}} {{rust_os}}
    
    # Create ISO directory structure
    /bin/mkdir -p build/iso/boot/grub
    cp {{kernel}} build/iso/boot/{{kernel}}
    cp {{grub_cfg}} build/iso/boot/grub/grub.cfg
    
    # Generate bootable ISO
    grub-mkrescue -o {{iso}} build/iso 2> /dev/null
    rm -rf build/iso

# Run task: Boot the OS using QEMU emulator
run:
    qemu-system-x86_64 -cdrom {{iso}}

# Debug task: Boot the OS using QEMU emulator in debug mode
debug:
    qemu-system-x86_64 -cdrom {{iso}} -serial stdio -d int,cpu_reset

# Clean task: Remove build artifacts and object files
clean:
    rm -rf build kernel.bin chaos.iso build bootloader/*.o
    cargo clean

