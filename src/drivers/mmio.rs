#[allow(dead_code)]
pub fn mmio_read8(addr:usize) -> u8 {
    unsafe { *(addr as *const u8)}
}

#[allow(dead_code)]
pub fn mmio_write8(addr:usize, val:u8) -> () {
    unsafe { *(addr as *mut u8) = val};
}

#[allow(dead_code)]
pub fn mmio_read16(addr:usize) -> u16 {
    unsafe { *(addr as *const u16)}
}

#[allow(dead_code)]
pub fn mmio_write16(addr:usize, val:u16) {
    unsafe { *(addr as *mut u16) = val}
}

pub fn mmio_read32(addr:usize) -> u32 {
    unsafe{ *(addr as *const u32)}
}

pub fn mmio_write32(addr:usize, val:u32) -> () {
    unsafe { *(addr as *mut u32) = val};
}

#[allow(dead_code)]
pub fn mmio_read64(addr:usize) -> u64 {
    unsafe{ *(addr as *const u64)}
}

#[allow(dead_code)]
pub fn mmio_write64(addr:usize, val:u64) -> () {
    unsafe { *(addr as *mut u64) = val};
}