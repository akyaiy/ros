#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
#[allow(unused)]
use ros::{print, println};

#[no_mangle]
pub extern "C" fn _start() {
    println!("\t\tROS v0.1");
    
    #[cfg(test)]
    test_main();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    use core::arch::asm;
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

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	ros::test_panic_handler(info);
    #[allow(unreachable_code)]
	loop {}
}

