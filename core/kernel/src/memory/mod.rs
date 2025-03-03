use log::info;

mod address;
mod global_allocator;
mod frame_allocator;
mod page_table;

pub unsafe fn init() {
    unsafe {
        info!("Initializing Global heap allocator...");
        global_allocator::init_heap();
        info!("Initializing Frame allocator...");
        frame_allocator::init_frame_allocator();
    }
}