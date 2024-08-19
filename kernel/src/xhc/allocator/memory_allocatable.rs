use super::aligned_address::AlignedAddress;

pub trait MemoryAllocatable {
    unsafe fn allocate_with_align(
        &mut self,
        bytes: usize,
        align: usize,
        bounds: usize,
    ) -> Option<AlignedAddress>;

    unsafe fn free(&mut self, addr: u64, bytes: usize);
}