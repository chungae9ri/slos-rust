
use core::arch::global_asm;

global_asm!(include_str!("vectors.S"));

#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    crate::init_kernel()
}