// main.rs

#![no_std]
#![no_main]

// panic handler
mod panic;

// printing to vga buffer
mod vga;

// interfacing with qemu
mod qemu;
mod serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, world!");
    serial_println!("Hello, serial world{}", "!");

    loop {}
}
