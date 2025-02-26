#![no_std]
#![no_main]
#![allow(unsafe_op_in_unsafe_fn)]

extern crate panic_halt;
extern crate rlibc;

mod vga;

// We use these crates for safe register access and CPUID.
extern crate raw_cpuid;
extern crate x86_64;

use core::arch::x86_64::__cpuid;
use core::mem::size_of;
use core::ptr::write_volatile;
use lazy_static::lazy_static;
use raw_cpuid::CpuId;
use x86_64::PhysAddr;
use x86_64::instructions::hlt;
use x86_64::registers::control::{Cr0, Cr0Flags, Cr3, Cr3Flags, Cr4, Cr4Flags};
use x86_64::registers::model_specific::{Efer, EferFlags};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::structures::paging::PhysFrame;

//
// 1. The Multiboot Header (replacing header.asm)
//

#[repr(C, packed)]
struct EndTag {
    tag_type: u16,
    flags: u16,
    size: u32,
}

#[repr(C, packed)]
struct MultibootHeader {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: u32,
    end_tag: EndTag,
}

/// Place our multiboot header in a dedicated section so that the bootloader finds it.
#[used]
#[unsafe(link_section = ".multiboot_header")]
static MULTIBOOT_HEADER: MultibootHeader = {
    const MAGIC: u32 = 0xe85250d6;
    const ARCH: u32 = 0;
    const HEADER_LENGTH: u32 = size_of::<MultibootHeader>() as u32;
    // Checksum is chosen so that the sum of all four header dwords is 0.
    const CHECKSUM: u32 = 0u32.wrapping_sub(MAGIC.wrapping_add(ARCH).wrapping_add(HEADER_LENGTH));
    MultibootHeader {
        magic: MAGIC,
        architecture: ARCH,
        header_length: HEADER_LENGTH,
        checksum: CHECKSUM,
        // End tag: a zero tag followed by a size of 8.
        end_tag: EndTag {
            tag_type: 0,
            flags: 0,
            size: 8,
        },
    }
};

//
// 2. Bootloader 32-bit code (replacing main.asm)
//

// A 16KiB stack. (In a real setup you’ll want your crt0 to load ESP with the address of STACK’s top.)
// #[repr(align(16))]
// static mut STACK: [u8; 16 * 1024] = [0; 16 * 1024];
#[repr(align(16))]
struct AlignedStack([u8; 16 * 1024]);

static mut STACK: AlignedStack = AlignedStack([0; 16 * 1024]);

// Page tables for the identity mapping.
// We use a newtype that guarantees 4K alignment.
#[repr(align(4096))]
struct PageTable([u64; 512]);

// Allocate our three levels of page tables.
static mut PAGE_TABLE_L4: PageTable = PageTable([0; 512]);
static mut PAGE_TABLE_L3: PageTable = PageTable([0; 512]);
static mut PAGE_TABLE_L2: PageTable = PageTable([0; 512]);

/// The 32-bit bootloader entry point.
/// (Linker or a small assembly stub must call this function with the proper multiboot arguments.)
#[unsafe(no_mangle)]
pub extern "C" fn _start(multiboot_magic: u32, _multiboot_info: u32) -> ! {
    // (Your startup assembly should have set up the stack already.)
    check_multiboot(multiboot_magic);
    check_cpuid();
    check_long_mode();

    unsafe {
        setup_page_tables();
        enable_paging();
    }

    load_gdt();

    // Normally we would perform a far jump to 64-bit mode.
    // For this Rust-only version (and since we can’t easily emit a far jump without asm),
    // we simply call our long-mode entry function.
    long_mode_start()
}

/// Verify that the multiboot magic number is correct.
fn check_multiboot(magic: u32) {
    if magic != 0x36d76289 {
        error(b'M');
    }
}

/// Check that CPUID is available.
/// (Our check is simplified using the raw-cpuid crate.)
fn check_cpuid() {
    let cpuid = CpuId::new();
    if cpuid.get_vendor_info().is_none() {
        error(b'C');
    }
    // (In your assembly you toggled the ID flag; here we assume that if vendor_info exists, CPUID works.)
}

// fn check_long_mode() {
//     let cpuid = CpuId::new();
//     if let Some(ext_info) = cpuid.get_extended_function_info() {
//         if !ext_info.has_long_mode() {
//             error(b'L');
//         }
//     } else {
//         error(b'L');
//     }
// }

/// Check if CPU supports Long Mode (64-bit)
fn check_long_mode() {
    // CPUID with EAX = 0x80000001 (Extended Features)
    let cpuid = unsafe { __cpuid(0x80000001) };

    // Check if bit 29 (Long Mode) in EDX is set
    if cpuid.edx & (1 << 29) == 0 {
        error(b'L');
    }
}

/// Set up identity-mapped page tables using 2 MiB pages.
unsafe fn setup_page_tables() {
    // Set L4[0] to point to the L3 table (with present and writable flags).
    PAGE_TABLE_L4.0[0] = (&PAGE_TABLE_L3 as *const _ as u64) | 0b11;
    // Set L3[0] to point to the L2 table.
    PAGE_TABLE_L3.0[0] = (&PAGE_TABLE_L2 as *const _ as u64) | 0b11;

    // Map the first 1GiB of memory in L2 using 2MiB pages.
    for i in 0..512 {
        PAGE_TABLE_L2.0[i] = (i as u64 * 0x200000) | 0b10000011;
    }
}

/// Enable paging, PAE, and long mode.
unsafe fn enable_paging() {
    // Load our L4 table address into CR3.
    // Cr3::write((&PAGE_TABLE_L4 as *const _ as u64).into());
    let frame = PhysFrame::containing_address(PhysAddr::new(&PAGE_TABLE_L4 as *const _ as u64));
    Cr3::write(frame, Cr3Flags::empty());

    // Enable Physical Address Extension (PAE) in CR4.
    let mut cr4 = Cr4::read();
    cr4.insert(Cr4Flags::PHYSICAL_ADDRESS_EXTENSION);
    Cr4::write(cr4);

    // Enable long mode in the EFER MSR.
    let mut efer = Efer::read();
    efer.insert(EferFlags::LONG_MODE_ENABLE);
    Efer::write(efer);

    // Finally, enable paging in CR0.
    let mut cr0 = Cr0::read();
    cr0.insert(Cr0Flags::PAGING);
    Cr0::write(cr0);
}

lazy_static! {
    pub static ref GDT: GlobalDescriptorTable = {
        let mut gdt = GlobalDescriptorTable::new();
        gdt.append(Descriptor::kernel_code_segment());
        gdt
    };
}

/// Load a basic Global Descriptor Table (GDT).
fn load_gdt() {
    // Add a kernel code segment descriptor.
    GDT.load();
}

/// Write an error message (using VGA text memory) and halt.
fn error(code: u8) -> ! {
    unsafe {
        // Write “ERR: X” to VGA memory at 0xb8000.
        let vga_ptr = 0xb8000 as *mut u32;
        write_volatile(vga_ptr, 0x4F524F45); // “EROF” (little-endian representation)
        write_volatile(vga_ptr.add(1), 0x4F3A4F52); // “RO:O”
        write_volatile(vga_ptr.add(2), 0x4F204F20); // “ O O”
        // Write the error code (for example, 'M', 'C', or 'L').
        (0xb8000 as *mut u8).add(10).write_volatile(code);
    }
    loop {
        hlt();
    }
}

//
// 3. Long Mode 64-bit entry (replacing main64.asm)
//

/// Entry point after switching to 64-bit long mode.
/// (A proper far jump from the 32-bit code is required to transition; here we assume that has been done.)
#[unsafe(no_mangle)]
pub extern "C" fn long_mode_start() -> ! {
    // In long mode, segmentation is mostly disabled. Here we would normally zero DS, ES, etc.
    // (In our Rust code, we assume that the GDT is set and the CPU is in long mode.)

    // Call our kernel entry point.
    kernel_start()
}

// fn kernel_start() {
//     // Your kernel code starts here.
//     // For demonstration, we simply loop.
//     hlt_loop();
// }

/// Halt the CPU in an infinite loop.
fn hlt_loop() -> ! {
    loop {
        hlt();
    }
}

/// The kernel’s main entry point.
#[unsafe(no_mangle)]
pub extern "C" fn kernel_start() -> ! {
    println!("Poop");
    hlt_loop()
}
