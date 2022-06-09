#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rv64;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe fn main() {
	rv64::timer_test();
    loop {}
}
