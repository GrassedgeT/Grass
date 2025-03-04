use core::str;

use super::{address::{PhysPageNum, VirtPageNum}, frame_allocator::Frame};
use alloc::vec::Vec;
use alloc::vec;
use bitflags::*;

bitflags! {
    pub struct PTEFlags: u8 {
        const V = 1 << 0; // Valid
        const R = 1 << 1; // Readable
        const W = 1 << 2; // Writable
        const X = 1 << 3; // Executable
        const U = 1 << 4; // User
        const G = 1 << 5; // TODO!
        const A = 1 << 6; // Accessed
        const D = 1 << 7; // Dirty
    }
}

impl PartialEq for PTEFlags {
    fn eq(&self, other: &Self) -> bool {
        self.bits() == other.bits()
    }
    
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry {
    pub bits: usize,
}

impl PageTableEntry {
    /// Create a new page table entry with the given physical page number and flags
    pub fn new(ppn: PhysPageNum, flags: PTEFlags) -> Self {
        PageTableEntry {
            bits: ppn.0 << 10 | flags.bits() as usize,
        }
    }

    /// Create an empty page table entry
    pub fn empty() -> Self {
        PageTableEntry { bits: 0 }
    }

    /// Get the physical page number from a page table entry
    pub fn ppn(&self) -> PhysPageNum {
        (self.bits >> 10 & (1usize << 44) - 1).into()
    }

    /// Get the PTEflags from a page table entry
    pub fn flags(&self) -> PTEFlags {
        PTEFlags::from_bits(self.bits as u8).unwrap()
    }

    /// Check if the page table entry is valid
    pub fn is_valid(&self) -> bool {
        (self.flags() & PTEFlags::V) != PTEFlags::empty()
    }
}

pub struct PageTable {
    root_ppn: PhysPageNum,
    frames: Vec<Frame>
}

impl PageTable {
    /// Create a new page table
    pub fn new() -> Self {
        let frame = Frame::alloc().expect("Frame alloc fail: Out of memory");
        let root_ppn = frame.ppn.into();
        let frames = vec![frame];
        Self { root_ppn, frames }
    }

    /// Find the page table entry for the given virtual page number,
    /// or create a new one if it doesn't exist
    pub fn find_pte_or_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry>{
        let indexes = vpn.get_indexes();
        for (i, &index) in indexes.iter().enumerate() {
            
        }
    }


    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {

    }
    pub fn unmap(&self, vpn: VirtPageNum){

    }
}