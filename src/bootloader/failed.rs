use core::arch::{asm, naked_asm};

#[repr(C, packed)]
struct MultibootHeader {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: usize,
    end_tag: [u8; 8],
}

#[unsafe(link_section = ".multiboot_header_func")]
#[unsafe(no_mangle)]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0xe85250d6,
    architecture: 0,
    header_length: 24,
    checksum: 0x100000000 - (0xe85250d6 + 24),
    end_tag: [0, 0, 8, 0, 0, 0, 0, 0],
};

#[unsafe(no_mangle)]
pub extern "C" fn start() -> ! {
    unsafe {
        asm!(
            "mov esp, {stack_top}",
            "call check_multiboot",
            "call check_cpuid",
            "call check_long_mode",
            "call setup_page_tables",
            "call enable_paging",
            "lgdt [{gdt_ptr}]",  // Load GDT
            "jmp 0x08, {long_mode}",  // Far jump to 64-bit mode
            "hlt",
            long_mode = in(reg) long_mode_start,
            gdt_ptr = in(reg) &GDT_PTR,
            stack_top = const STACK_TOP,
        );
    };
    hlt_loop()
}

#[unsafe(no_mangle)]
#[allow(binary_asm_labels)]
pub extern "C" fn check_multiboot() {
    unsafe {
        asm!(
            "cmp eax, 0x36d76289",
            "jne 1f",
            "ret",
            "1:",
            "mov al, 'M'",
            "jmp error",
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[allow(binary_asm_labels)]
pub extern "C" fn check_cpuid() {
    unsafe {
        asm!(
            "pushfd",
            "pop eax",
            "mov ecx, eax",
            "xor eax, 1 << 21",
            "push eax",
            "popfd",
            "pushfd",
            "pop eax",
            "push ecx",
            "popfd",
            "cmp eax, ecx",
            "je 1f",
            "ret",
            "1:",
            "mov al, 'C'",
            "jmp error",
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
#[allow(binary_asm_labels)]
pub extern "C" fn check_long_mode() {
    unsafe {
        asm!(
            "mov eax, 0x80000000",
            "cpuid",
            "cmp eax, 0x80000001",
            "jb 1f",
            "mov eax, 0x80000001",
            "cpuid",
            "test edx, 1 << 29",
            "jz 1f",
            "ret",
            "1:",
            "mov al, 'L'",
            "jmp error",
            options(noreturn)
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn setup_page_tables() {
    unsafe {
        asm!(
            "mov eax, {page_table_l3}", "or eax, 0b11",
            "mov [{page_table_l4}], eax",
            "mov eax, {page_table_l2}", "or eax, 0b11",
            "mov [{page_table_l3}], eax",
            "mov ecx, 0",
            "2:",
            "mov eax, 0x200000", "mul ecx", "or eax, 0b10000011",
            "mov [{page_table_l2} + ecx * 8], eax",
            "inc ecx", "cmp ecx, 512", "jne 2b",
            "ret",
            page_table_l2 = const PAGE_TABLE_L2,
            page_table_l3 = const PAGE_TABLE_L3,
            page_table_l4 = const PAGE_TABLE_L4
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn enable_paging() {
    unsafe {
        asm!(
            "mov eax, {page_table_l4}", "mov cr3, eax",
            "mov eax, cr4", "or eax, 1 << 5", "mov cr4, eax",
            "mov ecx, 0xC0000080", "rdmsr", "or eax, 1 << 8", "wrmsr",
            "mov eax, cr0", "or eax, 1 << 31", "mov cr0, eax",
            "ret",
            page_table_l4 = const PAGE_TABLE_L4
        );
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn error() -> ! {
    unsafe {
        asm!(
            "mov dword ptr [0xb8000], 0x4f524f45",
            "mov dword ptr [0xb8004], 0x4f3a4f52",
            "mov dword ptr [0xb8008], 0x4f204f20",
            "mov byte ptr [0xb800a], al",
            "hlt",
            options(noreturn)
        );
    }
}

// Constants for memory locations
const PAGE_TABLE_L2: usize = 0x1000;
const PAGE_TABLE_L3: usize = 0x2000;
const PAGE_TABLE_L4: usize = 0x3000;
const STACK_TOP: usize = 0x8000;

#[unsafe(link_section = ".bss")]
static mut PAGE_TABLE_L4_DATA: [u8; 4096] = [0; 4096];
#[unsafe(link_section = ".bss")]
static mut PAGE_TABLE_L3_DATA: [u8; 4096] = [0; 4096];
#[unsafe(link_section = ".bss")]
static mut PAGE_TABLE_L2_DATA: [u8; 4096] = [0; 4096];
#[unsafe(link_section = ".bss")]
static mut STACK_DATA: [u8; 4096 * 4] = [0; 4096 * 4];

#[repr(C, packed)]
struct GdtPointer {
    limit: u16,
    base: u64,
}

#[unsafe(link_section = ".rodata")]
static GDT64_POINTER: GdtPointer = GdtPointer {
    limit: 16,
    base: 0x10000,
};

#[unsafe(no_mangle)]
pub extern "C" fn long_mode_start() -> ! {
    unsafe {
        asm!(
            "mov ax, 0",
            "mov ss, ax",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "call kernel_start",
            "hlt",
            options(noreturn)
        );
    }
}
