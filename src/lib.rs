#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

// extern crate panic_halt;
extern crate rlibc;

mod interrupts;
mod vga;

use core::panic::PanicInfo;

use bootloader_api::BootInfo;
use x86_64::{instructions::hlt, structures::gdt};

pub fn init() {
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Halt the CPU in an infinite loop.
fn hlt_loop() -> ! {
    loop {
        hlt();
    }
}

/// The kernel’s main entry point.
// #[unsafe(no_mangle)]
pub fn kernel_start(boot_info: &'static mut BootInfo) -> ! {
    println!("Chaos Cha-OS");
    #[cfg(test)]
    test_main();

    hlt_loop()
}

bootloader_api::entry_point!(kernel_start);

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
