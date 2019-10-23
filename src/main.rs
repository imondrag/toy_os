// main.rs

#![no_std]
#![no_main]

// panic handler
mod panic;

static MSG: &[u8] = b"Hello, world!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buf = 0xb8000 as *mut u16;

    for (i, &byte) in MSG.iter().enumerate() {
        let attr: u16 = 0x0f00;
        let colored_char: u16 = attr | byte as u16;

        unsafe {
            vga_buf.add(i).write(colored_char);
        }
    }

    loop {}
}
