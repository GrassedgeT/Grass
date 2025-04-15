//! This is virtual memory area module.
//! It defines the VmArea structure and function to manage it.

use alloc::collections::btree_map::BTreeMap;
use core::ops::Range;

use bitflags::bitflags;

use crate::{
    config::PAGE_SIZE,
    memory::{
        address::{PhysPageNum, VirtAddr, VirtPageNum},
        frame_allocator::Frame,
        page_table::{self, PTEFlags, PageTable},
    },
};

bitflags! {
    /// Map permission flags
    #[derive(Clone, PartialEq, Eq)]
    pub struct MapPermission: u8 {
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
    }
}

/// Map type
/// Direct: map the virtual address to the physical address directly
/// Framed: map the virtual address to the physical address by the frame number
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MapType {
    Direct,
    Framed,
}

/// Virtual memory area
#[derive(Clone)]
pub struct VmArea {
    vpns: Range<VirtPageNum>,
    frames_map: BTreeMap<VirtPageNum, Frame>,
    perm: MapPermission,
    map_type: MapType,
}

impl VmArea {
    pub fn new(start_va: VirtAddr, end_va: VirtAddr, map_type: MapType, perm: MapPermission) -> Self {
        let start_vpn = start_va.floor();
        let end_vpn = end_va.ceil();
        Self {
            vpns: start_vpn..end_vpn,
            frames_map: BTreeMap::new(),
            perm,
            map_type,
        }
    }

    pub fn start_vpn(&self) -> VirtPageNum {
        self.vpns.start
    }

    pub fn end_vpn(&self) -> VirtPageNum {
        self.vpns.end
    }

    pub fn from_another(another: &VmArea) -> Self {
        Self {
            vpns: another.vpns.clone(),
            frames_map: BTreeMap::new(),
            perm: another.perm.clone(),
            map_type: another.map_type.clone(),
        }
    }

    pub fn map_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        let ppn: PhysPageNum;
        match self.map_type {
            MapType::Direct => {
                ppn = PhysPageNum(vpn.0);
            }
            MapType::Framed => {
                let frame = Frame::alloc().expect("Frame alloc fail: Out of memory");
                ppn = frame.ppn;
                self.frames_map.insert(vpn, frame);
            }
        }
        let pte_flags = PTEFlags::from_bits(self.perm.bits()).unwrap();
        page_table.map(vpn, ppn, pte_flags);
    }

    pub fn map(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpns.clone() {
            self.map_one(page_table, vpn);
        }
    }

    pub fn unmap_one(&mut self, page_table: &mut PageTable, vpn: VirtPageNum) {
        match self.map_type {
            MapType::Direct => {}
            MapType::Framed => {
                self.frames_map.remove(&vpn);
            }
        }
        page_table.unmap(vpn);
    }

    pub fn unmap(&mut self, page_table: &mut PageTable) {
        for vpn in self.vpns.clone() {
            self.unmap_one(page_table, vpn);
        }
    }

    /// Copy data to the memory area
    /// data: start-aligned but maybe with shorter length
    /// assume that all frames were cleared before
    pub fn copy_data(&mut self, page_table: &mut PageTable, data: &[u8]) {
        assert_eq!(self.map_type, MapType::Framed);
        let mut start: usize = 0;
        let mut current_vpn = self.vpns.start;
        let len = data.len();
        loop {
            let src = &data[start..len.min(start + PAGE_SIZE)];
            let dst = &mut page_table.vpn2ppn(current_vpn).unwrap().get_bytes_mut()[..src.len()];
            dst.copy_from_slice(src);
            start += PAGE_SIZE;
            if start >= len {
                break;
            }
            current_vpn = VirtPageNum(current_vpn.0 + 1);
        }
    }
}
