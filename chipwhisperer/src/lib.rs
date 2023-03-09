#![no_std]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn test(left: u32, right: u32) -> u32 {
    let mut result: u32 = 0;

    for _ in 0..100 {
        result += left + right;
    }

    return result
}
