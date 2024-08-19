use super::allocator::memory_allocatable::MemoryAllocatable;

#[repr(transparent)]
pub struct ScratchpadBufferPtr(u64);

impl ScratchpadBufferPtr {
    pub fn new_with_allocate(
        address: u64,
        allocator: &mut impl MemoryAllocatable,
    ) -> Self {
        let mut me = Self::new(address);
        unsafe {
            me.allocate(allocator);
        }

        me
    }


    fn new(address: u64) -> Self {
        Self(address)
    }


    unsafe fn allocate(&mut self, allocator: &mut impl MemoryAllocatable) {
        let buff = allocator.try_allocate_device_context();

        *(self.0 as *mut u64) = buff;
    }
}
