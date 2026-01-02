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
    write!(vga_buffer::WRITER.lock(), "KERNEL PANIC: {}", _info.message()).unwrap();
    unsafe {
        asm!(
            "hlt"
        );
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!(vga_buffer::WRITER.lock(), "1\n").unwrap();
    panic!("panica bum bum '{}'", 123);
    loop {}
}