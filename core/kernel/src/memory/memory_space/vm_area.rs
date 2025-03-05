//! This is virtual memory area module.
//! It defines the VmArea structure and function to manage it.

use core::ops::Range;

use bitflags::bitflags;


bitflags! {
    /// Map permission flags
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
pub enum MapType {
    Direct,
    Framed
}

/// Virtual memory area
pub struct VmArea {
    vpns: Range<VirtPageNum>,
    perm: MapPermission,
    map_type: MapType,
}

