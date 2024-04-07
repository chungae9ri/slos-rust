#![no_main]
#![no_std]

mod arch;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe fn init_kernel() -> ! {
    loop {}
}