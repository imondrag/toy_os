#![feature(abi_x86_interrupt)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use bootloader::bootinfo::BootInfo;
use bootloader::entry_point;
use core::fmt::Write;
use core::panic::PanicInfo;
use toy_os::{gdt, interrupts, vga_buffer::Writer};

#[cfg(not(test))]
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };

    x86_64::instructions::interrupts::enable();

    // explicitly call breakpoint interrupt
    // should call interrupt handler and continue with program
    //x86_64::instructions::int3();

    writeln!(Writer, "WE GOOD!");

    for i in 0.. {
        if i % 1_000_000 == 0 {
            write!(Writer, "-");
        }
    }

    toy_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    writeln!(Writer, "{}", info);
    toy_os::hlt_loop();
}
