#![no_std]
#![no_main]
#![feature(asm)]

extern crate x86_64;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Access hardware directly using x86_64 crate
    let cr0: u64;
    unsafe { asm!("mov rax, cr0" : "=r"(cr0)) };

    // Modify a system register (not recommended for most use cases)
    let new_cr0 = cr0 | 0x80000000; // Enable paging
    unsafe { asm!("mov cr0, {}", in(reg) new_cr0) };

    loop {}
}
