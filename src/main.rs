#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

static DEBUG: bool = false;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
#[allow(unused)]
use ros::{err, hlt_loop, klog, ok, print, println, trace_execution};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use ros::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};

    //    crate::print_banner!();
    /* ***************************************** */
    ros::trace_execution!("Initialization", {
        ros::init();

        let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

        let mapper = unsafe { memory::init(phys_mem_offset) };

        let addresses = [
            0xb8000,
            0x201008,
            0x0100_0020_1a10,
            boot_info.physical_memory_offset,
        ];

        for &address in &addresses {
            let virt = VirtAddr::new(address);
            let phys = mapper.translate_addr(virt);
            klog!("{:?} -> {:?}", virt, phys)
        }

        ok!()
    });

    /* ***************************************** */

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

#[macro_export]
macro_rules! print_banner {
    () => {
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
    };
}
