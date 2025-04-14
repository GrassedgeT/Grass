//! Definition and conversion functions for physical and virtual addresses.
use core::{iter::Step, ops::Add};

use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};

use super::page_table::{PageTable, PageTableEntry};

const PA_WIDTH_SV39: usize = 56;
const VA_WIDTH_SV39: usize = 39;
const PPN_WIDTH_SV39: usize = PA_WIDTH_SV39 - PAGE_SIZE_BITS;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PhysAddr(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VirtAddr(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PhysPageNum(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct VirtPageNum(pub usize);

// "Phys" asociated implement
impl From<usize> for PhysAddr {
    fn from(phys_addr: usize) -> Self {
        Self(phys_addr & ((1 << PA_WIDTH_SV39) - 1))
    }
}

impl From<usize> for PhysPageNum {
    fn from(value: usize) -> Self {
        Self(value & ((1 << PPN_WIDTH_SV39) - 1))
    }
}

impl From<PhysAddr> for usize {
    fn from(phys_addr: PhysAddr) -> Self {
        phys_addr.0
    }
}

impl From<PhysPageNum> for usize {
    fn from(ppn: PhysPageNum) -> Self {
        ppn.0
    }
}

impl PhysAddr {
    /// get the offset of the address in the page
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }

    /// get the page number of the address
    pub fn floor(&self) -> PhysPageNum {
        PhysPageNum(self.0 >> PAGE_SIZE_BITS)
    }

    /// get the next page number of the address
    pub fn ceil(&self) -> PhysPageNum {
        PhysPageNum((self.0 + PAGE_SIZE - 1) >> PAGE_SIZE_BITS)
    }
}

impl PhysPageNum {
    /// Return a array of PageTableEntry in the page 
    pub fn get_ptes_mut(&self) -> &'static mut [PageTableEntry] {
        let phys_addr: PhysAddr = self.clone().into();
        unsafe {
            // 64 bytes per PageTableEntry
            core::slice::from_raw_parts_mut(phys_addr.0 as *mut PageTableEntry, PAGE_SIZE / 64)
        }
    }
    
    /// Return a array of u8 in the page
    pub fn get_bytes_mut(&self) -> &'static mut [u8] {
        let phys_addr: PhysAddr = self.clone().into();
        unsafe {
            core::slice::from_raw_parts_mut(phys_addr.0 as *mut u8, PAGE_SIZE)
        }
    }
    
    /// Return a mutable reference of a specific type in the page
    pub fn get_mut<T>(&self) -> &'static mut T {
        let phys_addr: PhysAddr = self.clone().into();
        unsafe {
            (phys_addr.0 as *mut T).as_mut().unwrap()
        }
    }
}


impl From<PhysAddr> for PhysPageNum {
    fn from(phys_addr: PhysAddr) -> Self {
        phys_addr.floor()
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(ppn: PhysPageNum) -> Self {
        Self(ppn.0 << PAGE_SIZE_BITS)
    }
}


// "Virt" asociated implement
impl From<VirtAddr> for usize {
    fn from(virt_addr: VirtAddr) -> Self {
        virt_addr.0
    }
}

impl From<usize> for VirtAddr {
    fn from(addr: usize) -> Self {
        Self(addr & ((1 << VA_WIDTH_SV39) - 1))
    }
}

impl Add<usize> for VirtAddr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Step for VirtPageNum {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        let steps = end.0.checked_sub(start.0);
        (steps.unwrap_or(0), steps)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Some(Self(start.0.checked_add(count)?))
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Some(Self(start.0.checked_sub(count)?))
    }
}

impl VirtPageNum {
    /// get the three indexes of the page table, 9 bits each 
    /// 0: level 2 index 
    /// 1: level 1 index
    /// 2: level 0 index
    pub fn get_idxs(&self) -> [usize; 3] {
        let mut vpn = self.0;
        let mut indexes = [0; 3];
        for i in (0..3).rev() {
            indexes[i] = vpn & 511;
            vpn >>= 9;
        }
        indexes
    }
}

impl VirtAddr {
    pub fn floor(&self) -> VirtPageNum {
        VirtPageNum(self.0 >> PAGE_SIZE_BITS)
    }
    pub fn ceil(&self) -> VirtPageNum {
        VirtPageNum((self.0 + PAGE_SIZE - 1) >> PAGE_SIZE_BITS)
    }
    pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    pub fn aligned(&self) -> bool {
        self.page_offset() == 0
    }
}