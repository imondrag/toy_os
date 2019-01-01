#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

use bootloader::{entry_point, bootinfo::BootInfo};
use core::panic::PanicInfo;
use core::fmt::Write;
use toy_os::{gdt, interrupts, vga_buffer::Writer};

#[cfg(not(test))]
entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    gdt::init();
    interrupts::init_idt();

    unsafe { interrupts::PICS.lock().initialize() };

    // mask timer interrupts
    interrupts::irq_set_mask(0);

    x86_64::instructions::interrupts::enable();

    // explicitly call breakpoint interrupt
    // should call interrupt handler and continue with program
    //x86_64::instructions::int3();

    writeln!(Writer, "Hello world!");
    let msr = x86_64::registers::model_specific::Efer;
    writeln!(Writer, "These are my flags: {:x}", msr);
    toy_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    writeln!(Writer, "{}", info);
    toy_os::hlt_loop();
}
