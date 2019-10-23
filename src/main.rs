// main.rs

#![no_std]
#![no_main]

// panic handler
mod panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
