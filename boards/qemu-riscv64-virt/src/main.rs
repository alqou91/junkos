#![no_std]
#![no_main]

use rv64;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe fn main() {
	loop {}
}
