#![no_main]
#![no_std]

mod arch;
mod drivers;
mod print;

use core::panic::PanicInfo;
const UART_BASE_ADDR:usize = 0x8000000;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe fn init_kernel() -> ! {
    let mut uart_inst = drivers::uart::uart::StdioUart::new(UART_BASE_ADDR);
    println!(&mut uart_inst, "Hello World\n");
    loop {}
}