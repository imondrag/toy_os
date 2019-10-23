// panic.rs: panic-halt -- halt the thread on panic; messages are discarded

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
