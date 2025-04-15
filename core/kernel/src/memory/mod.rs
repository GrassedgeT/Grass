use log::info;
use memory_space::{KERNEL_SPACE, remap_test};

mod address;
mod frame_allocator;
mod global_allocator;
mod memory_space;
mod page_table;

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
