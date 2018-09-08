#![feature(panic_handler)]
#![no_std]
#![no_main]

extern crate bootloader_precompiled;

use core::panic::PanicInfo;

mod vga_buffer;

// This function is called on panic.
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

// Defining a new entry place
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
