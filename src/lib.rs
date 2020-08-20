#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(const_in_array_repeat_expressions)]
#![feature(wake_trait)]
#![feature(thread_local)]

extern crate alloc;
extern crate rlibc;

pub mod kernel;
pub mod qemu;
pub mod task;

pub use bootloader::BootInfo;
use kernel::memory::BootInfoFrameAllocator;
use x86_64::VirtAddr;

/// This is the kernel entry point for the primary CPU.
/// TODO: THIS SHOULD NOT RETURN
pub fn kmain(boot_info: &'static BootInfo) {
    kernel::gdt::init();
    kernel::interrupts::init_idt();
    unsafe { kernel::interrupts::PICS.lock().initialize() };

    x86_64::instructions::interrupts::enable();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { kernel::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    kernel::allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");
}

/// This is the main kernel entry point for secondary CPUs
#[allow(unreachable_code, unused_variables)]
pub fn kmain_ap(id: usize) -> ! {
    hlt_loop();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[macro_export]
macro_rules! userspace_entrypoint {
    ($path:path) => {
        #[export_name = "_start"]
        pub extern "C" fn __impl_start(boot_info: &'static $crate::BootInfo) -> ! {
            // validate the signature of the program entry point
            let f: fn() -> ! = $path;
            $crate::kmain(boot_info);
            f()
        }
    };
}
