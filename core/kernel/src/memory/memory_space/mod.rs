//! This is the memory space module. 
//! It contains MemorySpace assoicated items.

use alloc::{collections::btree_map::BTreeMap};
use alloc::sync::Arc;
use vm_area::{MapPermission, VmArea};

use super::address::VirtAddr;
use super::{address::VirtPageNum, page_table::PageTable};
use crate::sync::safe_cell::SafeCell;
pub mod vm_area;

unsafe extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sstack();
    fn estack();
    fn sbss();
    fn ebss();
    fn ekernel();
    fn strampoline();
}

lazy_static! {
    /// The kernel memory space
    pub static ref KERNEL_SPACE: Arc<SafeCell<MemorySpace>> = {
        unsafe {
            Arc::new(SafeCell::new(MemorySpace::new_kernel()))
        }
    }; 
}

/// Get the kernel memory space's satp token
pub fn kernel_satp() -> usize {
    KERNEL_SPACE.exclusive_access().page_table.satp_token()
}

/// Memory Space is the abstraction of a process's virtual memory.
/// It contains the page table and a collection of the virtual memory areas.
pub struct MemorySpace {
    page_table: PageTable,
    areas: BTreeMap<VirtPageNum, VmArea>,
}

impl MemorySpace {
    
    /// Create a new empty memory space
    pub fn new_bare() -> Self {
        Self {
            page_table: PageTable::new(),
            areas: BTreeMap::new(),
        }
    }

    /// Get the satp token of the memory space
    pub fn satp_token(&self) -> usize {
        self.page_table.satp_token()
    }

    /// Assume that no conflict
    // TODO! check the conflict
    pub fn insert_framed_area(
        &mut self, 
        start_va: VirtAddr, 
        end_va: VirtAddr, 
        perm: MapPermission
    ) {
        self.push(
            VmArea::new(start_va, end_va, vm_area::MapType::Framed, perm), 
            None
        );
    }

    pub fn remove_area_with_start_vpn(&mut self, start_vpn: VirtPageNum) {
        let area = self.areas.remove(&start_vpn);
        if let Some(mut area) = area {
            area.unmap(&mut self.page_table);
        }
    }

    /// Push a new vm area into the memory space
    fn push(&mut self, mut vm_area: VmArea, data: Option<&[u8]>) {
        VmArea::map(&mut vm_area, &mut self.page_table);
        if let Some(data) = data {
            vm_area.copy_data(&mut self.page_table, data);
        }
        self.areas.insert(vm_area.start_vpn().clone(), vm_area);
    }
    
    /// Create a new kernel memory space
    pub fn new_kernel() -> Self {
        todo!()
    }
}