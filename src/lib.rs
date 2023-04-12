#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate panic_halt;
extern crate rlibc;

mod vga;

#[no_mangle]
pub extern "C" fn kernel_start() -> ! {
    println!("Poop");
    loop {}
}
