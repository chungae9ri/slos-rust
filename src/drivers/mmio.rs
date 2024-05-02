#[cfg(target_arch = "aarch64")]
pub type ArchAddr = u64;

#[cfg(not(target_arch = "aarch64"))]
pub type ArchAddr = u32;

#[allow(dead_code)]
pub fn mmio_read8(addr:ArchAddr) -> u8 {
    unsafe { *(addr as *const u8)}
}

#[allow(dead_code)]
pub fn mmio_write8(addr:ArchAddr, val:u8) -> () {
    unsafe { *(addr as *mut u8) = val};
}

#[allow(dead_code)]
pub fn mmio_read16(addr:ArchAddr) -> u16 {
    unsafe { *(addr as *const u16)}
}

#[allow(dead_code)]
pub fn mmio_write16(addr:ArchAddr, val:u16) {
    unsafe { *(addr as *mut u16) = val}
}

pub fn mmio_read32(addr:ArchAddr) -> u32 {
    unsafe{ *(addr as *const u32)}
}

pub fn mmio_write32(addr:ArchAddr, val:u32) -> () {
    unsafe { *(addr as *mut u32) = val};
}

#[allow(dead_code)]
pub fn mmio_read64(addr:ArchAddr) -> u64 {
    unsafe{ *(addr as *const u64)}
}

#[allow(dead_code)]
pub fn mmio_write64(addr:ArchAddr, val:u64) -> () {
    unsafe { *(addr as *mut u64) = val};
}