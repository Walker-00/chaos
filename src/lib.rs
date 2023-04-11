#![feature(lang_items)]
#![no_std]
#![no_main]

extern crate rlibc;

use core::panic::PanicInfo;

static TEXT: &[u8] = b"Hello";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern fn kernel_start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in TEXT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn eh_personality() {}
