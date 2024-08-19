use super::aligned_address::AlignedAddress;

pub trait MemoryAllocatable {
    unsafe fn allocate_with_align(
        &mut self,
        bytes: usize,
        align: usize,
        bounds: usize,
    ) -> Option<AlignedAddress>;

    unsafe fn free(&mut self, addr: u64, bytes: usize);

    fn try_allocate_trb_ring(&mut self, ring_size: usize) -> u64 {
        self.try_allocate_with_align(core::mem::size_of::<u128>() * ring_size, 64, 4096).address()
    }

    fn try_allocate_device_context_array(&mut self, max_slots: u8) -> u64 {
        self.try_allocate_with_align(core::mem::size_of::<u64>() * max_slots as usize, 64, 4096).address()
    }

    fn try_allocate_input_context(&mut self) -> u64 {
        self.try_allocate_with_align(core::mem::size_of::<xhci::context::Input32Byte>(), 64, 0).address()
    }

    fn try_allocate_device_context(&mut self) -> u64 {
        self.try_allocate_with_align(core::mem::size_of::<xhci::context::Device32Byte>(), 64, 0).address()
    }

    fn try_allocate_max_scratchpad_buffers(&mut self, len: usize) -> u64 {
        self.try_allocate_with_align(core::mem::size_of::<u64>() * len, 4096, 4096).address()
    }

    fn try_allocate_with_align(
        &mut self,
        bytes: usize,
        align: usize,
        bounds: usize,
    ) -> AlignedAddress {
        unsafe {
            self.allocate_with_align(bytes, align, bounds).expect("Not enough memory")
        }
    }
}