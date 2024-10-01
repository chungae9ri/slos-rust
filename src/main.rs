#![no_main]
#![no_std]

mod arch;
mod drivers;
mod print;
mod device_driver;

use core::panic::PanicInfo;
use core::time::Duration;

use device_driver::DriverManager;
use device_driver::DeviceDriverDesc;
use arch::aarch64::time::spin_for;

const UART_BASE_ADDR:usize = 0xFF010000;

pub static mut UART_INST:drivers::uart::uart::Uart = drivers::uart::uart::Uart::new(UART_BASE_ADDR);
pub static mut DRV_MANAGER_INST:DriverManager = DriverManager::new();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe fn init_kernel() -> ! {
    UART_INST.init_base_addr(UART_BASE_ADDR);

    let uart_desc:DeviceDriverDesc = DeviceDriverDesc::new(&UART_INST);

    DRV_MANAGER_INST.register_driver_desc(uart_desc);

    let _ = uart_desc.device_driver.init();

    let mut i:u64 = 0;
    loop {
        println!("Hello World: {}\n", i);
        spin_for(Duration::from_millis(1000));
        i += 1;
    }
}