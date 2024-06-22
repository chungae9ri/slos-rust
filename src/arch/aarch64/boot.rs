use core::arch::global_asm;

global_asm!(include_str!("vectors.S"));

extern "C" {
    static _sbss: u8;
    static _ebss: u8;
}

#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    let mut bss_start_addr:usize;
    let mut bss_end_addr:usize;

    unsafe {
        bss_start_addr = &_sbss as *const u8 as usize;
        bss_end_addr = &_ebss as *const u8 as usize;
    }

    // initialize the .bss section
    for addr in bss_start_addr..bss_end_addr {
        unsafe { *(addr as *mut u8) = 0x0 };
    }
    crate::init_kernel()
}