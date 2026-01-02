#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
#[allow(unused)]
use ros::{print, println};

#[macro_export]
macro_rules! klog {
    ($($arg:tt)*) => (ros::print!(" :: {}\n", format_args!($($arg)*)));
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
//     print!(r#"
//   _____   ____   _____ 
//  |  __ \ / __ \ / ____|
//  | |__) | |  | | (___  
//  |  _  /| |  | |\___ \ 
//  | | \ \| |__| |____) |
//  |_|  \_\\____/|_____/ 
                       
//         ROS v0.1

// "#);
//     /* ***************************************** */

    klog!("Initializating");
    ros::init();

    /* ***************************************** */
    x86_64::instructions::interrupts::int3();
    #[cfg(test)]
    test_main();
    loop {}
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

