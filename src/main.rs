#![no_std]
#![no_main]
#![feature(box_syntax)]

extern crate alloc;

use toy_os::println;
use toy_os::task::{executor::Executor, keyboard, Task};
use toy_os::userspace_entrypoint;

userspace_entrypoint!(userspace_main);

fn userspace_main() -> ! {
    println!("Hello World!");

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();
}
