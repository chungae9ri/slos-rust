#![no_main]
#![no_std]

mod arch;
mod drivers;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe fn init_kernel() -> ! {
    drivers::uart::uart::init_uart();
    loop {}
}