#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use core::arch::asm;

use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "cli"
        );
    }
    print!("KERNEL PANIC: {}", _info.message());
    unsafe {
        asm!(
            "hlt"
        );
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hmmm");
    panic!("panica bum bum '{}'", 123);
    loop {}
}