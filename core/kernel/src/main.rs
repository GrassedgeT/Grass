//! The main entry of the kernel

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(step_trait)]
extern crate alloc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
mod console;
mod lang_items;
mod logger;
mod sbi;
mod sync;
pub mod memory;
mod config;
#[path ="boards/qemu.rs"]
mod board;

use core::arch::global_asm;
use log::info;

global_asm!(include_str!("boot/entry.asm"));

/// Entry point of kernel
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init();
    unsafe {
        memory::init();
    }
    info!("Hello, world!");
    panic!("shutdown");
}

/// Clear the .bss section
fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        (sbss as usize..ebss as usize).for_each(|a| {
            (a as *mut u8).write_volatile(0);
        });
    }
}
