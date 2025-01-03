#![no_std]
#![no_main]

#[macro_use]
mod console;
mod lang_items;
mod sbi;
mod logger;

use core::arch::global_asm;

global_asm!(include_str!("boot/entry.asm"));
#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init();
    println!("Hello, world!");
    panic!("shutdown");
}

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