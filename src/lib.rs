#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![no_std]
#![no_main]

// extern crate panic_halt;
extern crate rlibc;

mod vga;
use core::panic::PanicInfo;

use x86_64::instructions::hlt;

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
#[unsafe(no_mangle)]
pub extern "C" fn kernel_start() -> ! {
    println!("Poop");
    hlt_loop()
}
