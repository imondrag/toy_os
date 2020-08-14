#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(const_in_array_repeat_expressions)]
#![feature(wake_trait)]
#![feature(thread_local)]

extern crate alloc;
extern crate rlibc;

pub mod allocator;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod panic;
pub mod qemu;
pub mod serial;
pub mod task;
pub mod vga;

use crate::memory::BootInfoFrameAllocator;
use bootloader::BootInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use x86_64::VirtAddr;

/// A unique number that identifies the current CPU - used for scheduling
#[thread_local]
static CPU_ID: AtomicUsize = AtomicUsize::new(0);

/// Get the current CPU's scheduling ID
#[inline(always)]
pub fn cpu_id() -> usize {
    CPU_ID.load(Ordering::Relaxed)
}

/// The count of all CPUs that can have work scheduled
static CPU_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Get the number of CPUs currently active
#[inline(always)]
pub fn cpu_count() -> usize {
    CPU_COUNT.load(Ordering::Relaxed)
}

/// This is the kernel entry point for the primary CPU.
pub fn kmain(boot_info: &'static BootInfo) {
    // CPU_ID.store(0, Ordering::SeqCst);
    // CPU_COUNT.store(1, Ordering::SeqCst);

    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };

    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}

/// This is the main kernel entry point for secondary CPUs
#[allow(unreachable_code, unused_variables)]
pub fn kmain_ap(id: usize) -> ! {
    CPU_ID.store(id, Ordering::SeqCst);
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
