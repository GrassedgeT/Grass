//! This is the memory space module.
//! It contains MemorySpace assoicated items.

use alloc::{collections::btree_map::BTreeMap, sync::Arc};
use core::{arch::asm, mem};

use riscv::register::satp::{self, Satp};
use vm_area::{MapPermission, VmArea};
use xmas_elf::program;

use super::{
    address::{PhysAddr, VirtAddr, VirtPageNum},
    page_table::{PTEFlags, PageTable},
};
use crate::{
    board::MMIO,
    config::{MEMORY_END, PAGE_SIZE, TRAMPOLINE},
    sync::safe_cell::SafeCell,
};
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
    pub fn insert_framed_area(&mut self, start_va: VirtAddr, end_va: VirtAddr, perm: MapPermission) {
        self.push(VmArea::new(start_va, end_va, vm_area::MapType::Framed, perm), None);
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

    /// Map the Trampoline page at the top of the kernel memory space
    /// The trampoline page is used to switch between kernel and user space
    /// Mention that the trampoline page is not collected by areas
    pub fn map_trampoline(&mut self) {
        self.page_table.map(
            VirtAddr::from(TRAMPOLINE).into(),
            PhysAddr::from(strampoline as usize).into(),
            PTEFlags::R | PTEFlags::X,
        );
    }

    /// Create kernel memory space
    pub fn new_kernel() -> Self {
        let mut kernel_space = Self::new_bare();

        // Map trampoline page
        kernel_space.map_trampoline();

        // Map kernel sections
        println!(".text [{:#x}, {:#x}]", stext as usize, etext as usize);
        println!(".rodata [{:#x}, {:#x}]", srodata as usize, erodata as usize);
        println!(".data [{:#x}, {:#x}]", sdata as usize, edata as usize);
        println!(".bss [{:#x}, {:#x}]", sbss as usize, ebss as usize);
        println!(".stack [{:#x}, {:#x}]", sstack as usize, estack as usize);
        println!("mapping text section");
        kernel_space.push(
            VmArea::new(
                VirtAddr::from(stext as usize),
                VirtAddr::from(etext as usize),
                vm_area::MapType::Direct,
                MapPermission::R | MapPermission::X,
            ),
            None,
        );
        println!("mapping rodata section");
        kernel_space.push(
            VmArea::new(
                VirtAddr::from(srodata as usize),
                VirtAddr::from(erodata as usize),
                vm_area::MapType::Direct,
                MapPermission::R,
            ),
            None,
        );

        println!("mapping data section");
        kernel_space.push(
            VmArea::new(
                VirtAddr::from(sdata as usize),
                VirtAddr::from(edata as usize),
                vm_area::MapType::Direct,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        println!("mapping stack section");
        kernel_space.push(
            VmArea::new(
                VirtAddr::from(sstack as usize),
                VirtAddr::from(estack as usize),
                vm_area::MapType::Direct,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        println!("mapping bss section");
        kernel_space.push(
            VmArea::new(
                VirtAddr::from(sbss as usize),
                VirtAddr::from(ebss as usize),
                vm_area::MapType::Direct,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        // Map whole physical memory make the kernel can access all physical memory directly
        println!("mapping physical memory");
        kernel_space.push(
            VmArea::new(
                VirtAddr::from(ekernel as usize),
                VirtAddr::from(MEMORY_END),
                vm_area::MapType::Direct,
                MapPermission::R | MapPermission::W,
            ),
            None,
        );
        println!("mapping memory-mapped registers");
        for pair in MMIO {
            kernel_space.push(
                VmArea::new(
                    VirtAddr::from((*pair).0),
                    VirtAddr::from((*pair).0 + (*pair).1),
                    vm_area::MapType::Direct,
                    MapPermission::R | MapPermission::W,
                ),
                None,
            );
        }
        kernel_space
    }

    /// Map content from elf file to the memory space
    /// returns user_sp_base and entry point
    pub fn from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        let mut memory_set = Self::new_bare();
        // map the trampoline page
        memory_set.map_trampoline();
        // map program content with U flag
        let elf = xmas_elf::ElfFile::new(elf_data).unwrap();
        let elf_header = elf.header;
        let magic = elf_header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "Invalid ELF file");
        let mut max_end_vpn = VirtPageNum(0);
        elf.program_iter().for_each(|ph| {
            if let Ok(p_type) = ph.get_type() {
                match p_type {
                    xmas_elf::program::Type::Load => {
                        let start_va: VirtAddr = (ph.virtual_addr() as usize).into();
                        let end_va: VirtAddr = ((ph.virtual_addr() + ph.mem_size()) as usize).into();
                        let mut map_perm = MapPermission::U;
                        let ph_flags = ph.flags();
                        if ph_flags.is_read() {
                            map_perm |= MapPermission::R;
                        }
                        if ph_flags.is_write() {
                            map_perm |= MapPermission::W;
                        }
                        if ph_flags.is_execute() {
                            map_perm |= MapPermission::X;
                        }
                        let area = VmArea::new(start_va, end_va, vm_area::MapType::Framed, map_perm);
                        max_end_vpn = area.end_vpn();
                        memory_set.push(
                            area,
                            Some(&elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize]),
                        );
                    }
                    _ => {}
                }
            }
        });
        let max_end_va: VirtAddr = max_end_vpn.into();
        let mut user_stack_base: usize = max_end_va.into();
        user_stack_base += PAGE_SIZE;

        (memory_set, user_stack_base, elf.header.pt2.entry_point() as usize)
    }

    /// Create a new memory space from an existed user space
    /// Copy the data sections, user stack and trap context
    pub fn from_existed_user(user_space: &MemorySpace) -> Self {
        let mut memory_set = Self::new_bare();
        // map the trampoline page
        memory_set.map_trampoline();
        // copy data sections/trap_context/user_stack
        for area in user_space.areas.values() {
            let new_area = VmArea::from_another(area);
            memory_set.push(new_area, None);
            for vpn in area.start_vpn()..area.end_vpn() {
                let src_ppn = user_space.page_table.vpn2ppn(vpn).unwrap();
                let dst_ppn = memory_set.page_table.vpn2ppn(vpn).unwrap();
                dst_ppn.get_bytes_mut().copy_from_slice(&src_ppn.get_bytes_mut());
            }
        }
        memory_set
    }

    /// Activate current memory space
    /// use to change memory space
    pub fn activate(&self) {
        let satp = Satp::from_bits(self.satp_token());
        unsafe {
            satp::write(satp);
            asm!("sfence.vma");
        }
    }

    pub fn recycle_data_pages(&mut self) {
        self.areas.clear();
    }

    pub fn kernel_copy() -> Self {
        let areas = KERNEL_SPACE.exclusive_access().areas.clone();
        Self {
            page_table: PageTable::from_satp(kernel_satp()),
            areas: areas,
        }
    }
}

#[allow(unused)]
pub fn remap_test() {
    let mut kernel_space = KERNEL_SPACE.exclusive_access();
    let mid_text: VirtAddr = ((stext as usize + etext as usize) / 2).into();
    let mid_rodata: VirtAddr = ((srodata as usize + erodata as usize) / 2).into();
    let mid_data: VirtAddr = ((sdata as usize + edata as usize) / 2).into();
    assert_eq!(
        !kernel_space.page_table.find_pte(mid_text.floor()).unwrap().flags() & PTEFlags::W,
        PTEFlags::W
    );
    assert_eq!(
        !kernel_space.page_table.find_pte(mid_rodata.floor()).unwrap().flags() & PTEFlags::W,
        PTEFlags::W
    );
    assert_eq!(
        !kernel_space.page_table.find_pte(mid_data.floor()).unwrap().flags() & PTEFlags::X,
        PTEFlags::X
    );
    println!("remap_test passed!");
}
