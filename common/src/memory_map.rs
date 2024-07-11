use core::slice;

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct MemoryMap {
    pub descriptors: *const Descriptor,
    pub descriptors_len: u64,
}

impl MemoryMap {
    pub fn descriptors(&self) -> &[Descriptor] {
        unsafe { slice::from_raw_parts(self.descriptors, self.descriptors_len as usize) }
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Descriptor {
    pub memory_type: u32,
    pub phys_start: u64,
    pub phys_end: u64,
    pub virt_start: u64,
    pub att: u64,
}