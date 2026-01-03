#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

static DEBUG: bool = false;

use core::panic::PanicInfo;
#[allow(unused)]
use ros::{err, hlt_loop, klog, ok, print, println, trace_execution};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!(
        r#"
  _____   ____   _____ 
 |  __ \ / __ \ / ____|
 | |__) | |  | | (___  
 |  _  /| |  | |\___ \ 
 | | \ \| |__| |____) |
 |_|  \_\\____/|_____/ 
                       
        ROS v0.1

"#
    );
    /* ***************************************** */
    ros::trace_execution!("Initialization", {
        ros::init();
        ok!()
    });

    /* ***************************************** */
    use core::arch::asm;
    unsafe {
        asm!("mov qword ptr [0xDEADBEEF], 0x42",);
    }
    #[cfg(test)]
    test_main();
    ros::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    x86_64::instructions::interrupts::disable();
    if DEBUG {
        print!("KERNEL PANIC: '{}'", info);
    } else {
        print!("KERNEL PANIC: '{}'", info.message());
    }
    x86_64::instructions::hlt();
    ros::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info);
    #[allow(unreachable_code)]
    ros::hlt_loop();
}
