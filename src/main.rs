// main.rs

#![no_std]
#![no_main]

// panic handler
mod panic;

static MSG: &[u8] = b"Hello, world!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buf = 0xb8000 as *mut u8;

    for (i, &byte) in MSG.iter().enumerate() {
        unsafe {
            *vga_buf.add(2 * i) = byte;
            *vga_buf.add(2 * i + 1) = 0xb;
        }
    }

    loop {}
}
