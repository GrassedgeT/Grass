//! This is the memory space module. 
//! It contains MemorySpace assoicated items.

use alloc::vec::Vec;
use vm_area::VmArea;

use super::page_table::PageTable;

pub mod vm_area;

pub struct MemorySpace {
    page_table: PageTable,
    areas: Vec<VmArea>,
}