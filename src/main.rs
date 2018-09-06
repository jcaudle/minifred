#![feature(panic_handler)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Defining a new entry place
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
