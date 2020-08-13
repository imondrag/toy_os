use crate::qemu::{exit_qemu, QemuExitCode};
use core::panic::PanicInfo;

pub fn test_panic_handler(_info: &PanicInfo) -> ! {
    // serial_println!("[failed]\n");
    // serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println!("{}", info);
    crate::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
