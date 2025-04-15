use log::info;
use memory_space::{remap_test, KERNEL_SPACE};

mod address;
mod global_allocator;
mod frame_allocator;
mod page_table;
mod memory_space;

pub unsafe fn init() {
    unsafe {
        info!("Initializing Global heap allocator...");
        global_allocator::init_heap();
        info!("Initializing Frame allocator...");
        frame_allocator::init_frame_allocator();
        info!("Initializing Kernel memory space...");
        KERNEL_SPACE.exclusive_access().activate();
        info!("test kernel space");
        remap_test();
    }
}