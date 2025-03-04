//!Define a frame allocator and assciated structs and functions, frames are managed by buddy_system_allocator crate 
use core::fmt::Debug;

use alloc::vec;
use log::info;

use crate::config::MEMORY_END;
use crate::memory::address::{PhysAddr, PhysPageNum};
use crate::sync::safe_cell::*;

///Define the basic behavior of a frame allocator
trait FrameAllocator {
    /// Allocate a frame of memory.
    fn alloc(&mut self, num: usize) -> Option<PhysPageNum>;

    /// Deallocate a frame of memory.
    fn dealloc(&mut self, frame: PhysPageNum, num: usize);
}

pub struct BuddyFrameAllocator {
    /// ORDER = 2^23 = 8MiB, same as max available memory defined in config.rs
    /// Also can change to ajust to different memory size
    /// TODO! Or may be my understanding is wrong, 
    allocator: buddy_system_allocator::FrameAllocator<23>,
}

impl BuddyFrameAllocator {
    /// Create a new frame allocator.
    pub fn new() -> Self {
        Self {
             allocator: buddy_system_allocator::FrameAllocator::new(),
        }
    }
}

lazy_static! {
    pub static ref FRAMEALLOCATOR: SafeCell<BuddyFrameAllocator> = {
        unsafe {
            SafeCell::new(BuddyFrameAllocator::new())
        }
    };
}



pub fn init_frame_allocator() {
    unsafe extern "C" {
        // It has been used below, 
        // but the compiler still reports that it is not used， 
        // FIXME! maybe a bug 
        #[allow(unused)]
        fn ekernel();
    }
    // Initialize the frame allocator, frame available from ekernel to MEMORY_END
    FRAMEALLOCATOR.exclusive_access()
                  .allocator
                  .add_frame(
                        PhysAddr::from(ekernel as usize).ceil().0,
                          PhysAddr::from(MEMORY_END).floor().0);
    frame_allocator_test();
}

impl FrameAllocator for BuddyFrameAllocator{
    fn alloc(&mut self, num: usize) -> Option<PhysPageNum> {
        self.allocator
            .alloc(num)
            .map(|ppn|{
                ppn.into()
            })
    }

    fn dealloc(&mut self, frame: PhysPageNum, num: usize) {
        self.allocator.dealloc(frame.0, num);
    }
}

/// A structure to manage a single physframe
#[derive(Debug)]
pub struct Frame{
    pub ppn: PhysPageNum,
}

impl Frame {
    pub fn alloc() -> Option<Self> {
        let mut allocator = FRAMEALLOCATOR.exclusive_access();
        let ppn = allocator.alloc(1).expect("Frame alloc fail: Out of memory");
        Some(Self { ppn })
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        let mut allocator = FRAMEALLOCATOR.exclusive_access();
        allocator.dealloc(self.ppn, 1);
    }
}

/* impl Debug for Frame {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Frame").field("ppn", &self.ppn.0).finish()
    }
} */

/// A structure to manage the physframes
/// it is a serial of frames beginning from ppn, num > 1
#[derive(Debug)]
pub struct Frames {
    pub ppn: PhysPageNum,
    pub num: usize,
}

impl Frames {
    pub fn alloc(num: usize) -> Option<Self> {
        let mut allocator = FRAMEALLOCATOR.exclusive_access();
        let ppn = allocator.alloc(num).expect("Frame alloc fail: Out of memory");
        Some(Self {ppn, num,})
    }
}

impl Drop for Frames {
    fn drop(&mut self) {
        let mut allocator = FRAMEALLOCATOR.exclusive_access();
        allocator.dealloc(self.ppn, self.num);
    }
}

/* impl Debug for Frames {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Frames").field("ppn", &self.ppn.0).field("num", &self.num).finish()
    }
}
 */
#[allow(unused)]
pub fn frame_allocator_test(){
    info!("Testing frame allocator...");
    let mut v = vec![];
    for i in 0..5 {
        let frame = Frame::alloc().expect("Frame alloc fail: Out of memory");
        info!("{:?} allocated", frame);
        v.push(frame);
    }
    v.clear();
    let mut v = vec![];
    for i in 0..5 {
        let frames = Frames::alloc(2).expect("Frames alloc fail: Out of memory");
        info!("{:?} allocated", frames);
        v.push(frames);
    }
    info!("Frame allocator test passed!");
}

