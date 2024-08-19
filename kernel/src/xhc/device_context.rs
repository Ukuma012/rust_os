use super::xhc_registers_operations::RegistersOperation;
use super::allocator::memory_allocatable::MemoryAllocatable;

pub trait DeviceContextOperations {
    fn write_device_context_array_addr(&mut self, device_context_addr: u64);

    fn setup_device_context_array(
        &mut self,
        device_slots: u8,
        scratchpad_buffers_len: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> DeviceContextArrayPtr  {
        todo!()
    }
}

impl<M> DeviceContextOperations for RegistersOperation<M>
where 
    M: xhci::accessor::Mapper + Clone,
{
    fn write_device_context_array_addr(&mut self, device_context_addr: u64) {
        self.registers_mut()
            .operational
            .dcbaap
            .update_volatile(|device_context| device_context.set(device_context_addr));
    }
}

#[repr(transparent)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct DeviceContextArrayPtr(u64);

impl DeviceContextArrayPtr {
    pub fn new(address: u64) -> Self {
        Self(address)
    }

    pub fn set_device_context_at(&mut self, index: usize, device_context_addr: u64) {
        unsafe {
            let ptr = (self.0 as *mut u64).add(index);

            ptr.write(device_context_addr);
        }
    }
}