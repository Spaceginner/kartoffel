use core::ptr;


// for some reason stdlib doesnt make those public, idk why
pub const MEM: *mut u32 = 0x08000000 as *mut u32;
pub const MEM_TIMER: *mut u32 = MEM;
pub const MEM_BATTERY: *mut u32 = MEM.wrapping_byte_add(1024);
pub const MEM_SERIAL: *mut u32 = MEM.wrapping_byte_add(2 * 1024);
pub const MEM_MOTOR: *mut u32 = MEM.wrapping_byte_add(3 * 1024);
pub const MEM_ARM: *mut u32 = MEM.wrapping_byte_add(4 * 1024);
pub const MEM_RADAR: *mut u32 = MEM.wrapping_byte_add(5 * 1024);
pub const MEM_COMPASS: *mut u32 = MEM.wrapping_byte_add(6 * 1024);


#[inline(always)]
pub unsafe fn read(ptr: *mut u32, off: usize) -> u32 {
    ptr::read_volatile(ptr.wrapping_add(off))
}

#[inline(always)]
pub unsafe fn write(ptr: *mut u32, off: usize, val: u32) {
    ptr::write_volatile(ptr.wrapping_add(off), val);
}
