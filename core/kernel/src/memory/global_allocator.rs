use buddy_system_allocator::LockedHeap;
use log::info;

use crate::config::KERNEL_HEAP_SIZE;

/// Global allocator for the kernel heap.
#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

/// Panic handler for heap allocation errors.
#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error: {:?}", layout);
}

static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

#[allow(static_mut_refs)]
pub unsafe fn init_heap() {
    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
        test_heap();
    }
}

#[allow(unused)]
fn test_heap() {
    info!("Testing heap...");
    use alloc::{boxed::Box, vec::Vec};
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    let bss_range = sbss as usize..ebss as usize;
    let a = Box::new(5);
    assert_eq!(*a, 5);
    assert!(bss_range.contains(&(a.as_ref() as *const _ as usize)));
    drop(a);
    let mut v: Vec<usize> = Vec::new();
    for i in 0..100 {
        v.push(i);
    }
    for (i, &val) in v.iter().enumerate() {
        assert_eq!(val, i);
    }
    assert!(bss_range.contains(&(v.as_ptr() as usize)));
    drop(v);
    info!("Heap test passed!");
}
