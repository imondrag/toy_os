#![feature(abi_x86_interrupt)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use bootloader::bootinfo::BootInfo;
use bootloader::entry_point;
use core::panic::PanicInfo;
use toy_os::{gdt, interrupts, println};

#[cfg(not(test))]
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    gdt::init();
    interrupts::init_idt();

    // explicitly call breakpoint interrupt
    // should call interrupt handler and continue with program
    x86_64::instructions::int3();

    // trigger a page fault
    unsafe {
        *(0xdeadbeef as *mut u64) = 42;
    };

    println!("Hello!");
    toy_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    toy_os::hlt_loop();
}
