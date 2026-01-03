#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

static DEBUG: bool = false;

use core::panic::PanicInfo;
#[allow(unused)]
use ros::{print, println};

#[macro_export]
macro_rules! klog {
    ($($arg:tt)*) => (ros::print!(" :: {}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! trace_execution {
    ($name:expr, $body:block) => {{
        klog!("[ {} ]", $name);
        let result = {
            $body
        };
        match &result {
            Ok(_) => klog!("[ {} ] done", $name),
            Err(e) => klog!("[ {} ] error: {:?}", $name, e)
        };
    }};
}

#[macro_export]
macro_rules! ok {
    () => { Ok::<(), ()>(()) };
}

#[macro_export]
macro_rules! err {
    () => { Err::<(), ()>(()) };
    ($e:expr) => { Err::<(), _>($e) };
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!(r#"
  _____   ____   _____ 
 |  __ \ / __ \ / ____|
 | |__) | |  | | (___  
 |  _  /| |  | |\___ \ 
 | | \ \| |__| |____) |
 |_|  \_\\____/|_____/ 
                       
        ROS v0.1

"#);
    /* ***************************************** */

    trace_execution!("Initialization", {
        ros::init();
        ok!()
    });

    /* ***************************************** */

    fn so() {
        so()
    }
    so();
    #[cfg(test)]
    test_main();
    loop {}
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
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	ros::test_panic_handler(info);
    #[allow(unreachable_code)]
	loop {}
}

