#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "cli"
        );
    }
    print!("KERNEL PANIC: {}", _info);
    unsafe {
        asm!(
            "hlt"
        );
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 1..30 {
        println!("hmmm {}", i);
    }
    panic!("panica bum bum '{}'", 123);
    loop {}
}