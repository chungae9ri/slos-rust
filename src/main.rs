#![no_main]
#![no_std]

mod arch;
mod drivers;
mod print;
mod device_driver;

use core::panic::PanicInfo;
use device_driver::DriverManager;
use crate::device_driver::DeviceDriverDesc;

const UART_BASE_ADDR:usize = 0xFF010000;

pub static mut UART_INST:drivers::uart::uart::Uart = unsafe {drivers::uart::uart::Uart::new(UART_BASE_ADDR)};
pub static mut DRV_MANAGER_INST:DriverManager = unsafe {DriverManager::new()};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe fn init_kernel() -> ! {
    // FIXME: This should be placed here even though the Uart::new()
    // initialize the base_addr. W/o this, the base_addr isn't valid value.
    UART_INST.init_base_addr(UART_BASE_ADDR);
    let uart_desc:DeviceDriverDesc = DeviceDriverDesc::new(&UART_INST);
    DRV_MANAGER_INST.register_driver_desc(uart_desc);
    let _ = uart_desc.device_driver.init();
    println!("Hello World\n");
    loop {}
}