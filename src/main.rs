#![feature(abi_x86_interrupt)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use bootloader::entry_point;
use core::panic::PanicInfo;
use toy_os::gdt;
use toy_os::init_idt;
use toy_os::println;

#[cfg(not(test))]
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static bootloader::bootinfo::BootInfo) -> ! {
    const NAME: &'static str = "Ivan";
    println!("Hello, {}!", NAME);

    gdt::init();
    init_idt();

    // explicitly call breakpoint interrupt
    // should call interrupt handler and continue with program
    x86_64::instructions::int3();

    println!("Passed!");
    toy_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    toy_os::hlt_loop();
}
