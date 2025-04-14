use core::str;

use super::{address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum}, frame_allocator::Frame};
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

    /// Get the root physical page number from satp register
    /// use this function when switching page table
    /// this page table doesn't have any frames
    pub fn from_satp(satp: usize) -> Self {
        Self {
            root_ppn: PhysPageNum::from(satp & ((1usize << 44) - 1)),
            frames: Vec::new(),
        }
    }



    /// Find the page table entry for the given virtual page number,
    /// or create a new one if it doesn't exist
    pub fn find_pte_or_create(&mut self, vpn: VirtPageNum) -> Option<&mut PageTableEntry>{
        let idxs = vpn.get_idxs();
        let mut ppn = self.root_ppn;
        let mut result = None;
        for (i, &index) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_ptes_mut()[idxs[i]];
            
            // reach the leaf node
            if i == 2 {
                result = Some(pte);
                break;
            }

            // if the next level page table(except the leaf node) doesn't exist, create it
            if !pte.is_valid() {
                let frame = Frame::alloc().expect("Frame alloc fail: Out of memory");
                *pte = PageTableEntry::new(frame.ppn, PTEFlags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }

    /// Find the page table entry for the given virtual page number
    /// or return None if it doesn't exist
    pub fn find_pte(&self, vpn: VirtPageNum) -> Option<&mut PageTableEntry> {
        let idxs = vpn.get_idxs();
        let mut ppn = self.root_ppn;
        let mut result = None;
        for (i, &index) in idxs.iter().enumerate() {
            let pte = &mut ppn.get_ptes_mut()[idxs[i]];
            if !pte.is_valid() {
                return None;
            }
            if i == 2 {
                result = Some(pte);
                break;
            }
            ppn = pte.ppn();
        }
        result
    }

    /// Map the given virtual page to the given physical page 
    pub fn map(&mut self, vpn: VirtPageNum, ppn: PhysPageNum, flags: PTEFlags) {
        let pte = self.find_pte_or_create(vpn).unwrap();
        debug_assert!(!pte.is_valid(), "Mapping an already mapped virt page");
        *pte = PageTableEntry::new(ppn, flags | PTEFlags::V);
    }

    /// Unmap the given virtual page 
    pub fn unmap(&self, vpn: VirtPageNum){
        let pte = self.find_pte(vpn).unwrap();
        debug_assert!(pte.is_valid(), "Unmapping an unmapped virt page");
        *pte = PageTableEntry::empty();
    }

    /// Translate the given virtual page number to the physical page number
    /// !!! Only used in the Framed map type
    pub fn vpn2ppn(&self, vpn: VirtPageNum) -> Option<PhysPageNum> {
        let pte = self.find_pte(vpn)?;
        if pte.is_valid() {
            Some(pte.ppn())
        } else {
            None
        }
    }

    /// Translate the given virtual address to the physical address
    /// !!! Only used in the Framed map type
    pub fn va2pa(&self, va: VirtAddr) -> Option<PhysAddr> {
        self.find_pte(va.clone().floor()).map(|pte| {
            let pa: PhysAddr = pte.ppn().into();
            let offset = va.page_offset();
            let pa_raw: usize = pa.into();
            (pa_raw + offset).into()
        })
    }

    /// Construct the satp token from the root physical page number,
    /// default enable SV39 mode
    pub fn satp_token(&self) -> usize {
        8usize << 60 | self.root_ppn.0
    }
}