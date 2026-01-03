#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unsafe_code)]

use core::panic::PanicInfo;

pub mod gdt;
pub mod interrupts;

pub mod qemu;
pub mod serial;
pub mod vga_buffer;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {
        interrupts::PICS.lock().initialize();
    };
    x86_64::instructions::interrupts::enable();
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[macro_export]
macro_rules! klog {
    ($($arg:tt)*) => (ros::print!(" :: {}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! trace_execution {
    ($name:expr, $body:block) => {{
        klog!("[ {} ]", $name);
        let result = { $body };
        match &result {
            Ok(_) => klog!("[ {} ] done", $name),
            Err(e) => klog!("[ {} ] error: {:?}", $name, e),
        };
    }};
}

#[macro_export]
macro_rules! ok {
    () => {
        Ok::<(), ()>(())
    };
}

#[macro_export]
macro_rules! err {
    () => {
        Err::<(), ()>(())
    };
    ($e:expr) => {
        Err::<(), _>($e)
    };
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
