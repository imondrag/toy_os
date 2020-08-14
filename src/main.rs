#![no_std]
#![no_main]
#![feature(box_syntax)]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use toy_os::println;
use toy_os::task::{executor::Executor, keyboard, Task};

entry_point!(userspace_main);

fn userspace_main(boot_info: &'static BootInfo) -> ! {
    // // TODO: kernel should be calling this userspace main function
    // // NOT how it's currently done
    toy_os::kmain(boot_info);
    println!("Hello World!");

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}
