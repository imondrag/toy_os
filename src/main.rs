// main.rs

#![no_std]
#![no_main]

// panic handler
mod panic;

// hanldes printing to vga buffer
mod vga_text;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");

    loop {}
}
