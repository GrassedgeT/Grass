pub const PAGE_SIZE: usize = 0x1000; //Page size = 4KiB
pub const PAGE_SIZE_BITS: usize = 0xc; //Page size = 12bits
pub const MEMORY_END: usize = 0x80800000; //Available memory From 0x80000000 to 0x80800000 = 8MiB
pub const KERNEL_HEAP_SIZE: usize = 0x200000; //Kernel heap size = 2MiB
