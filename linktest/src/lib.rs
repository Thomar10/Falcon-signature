#![no_std]

#[no_mangle]
pub extern "C" fn add(left: usize, right: usize) -> usize {
    left + right
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}