use core::cell::RefCell;

use alloc::rc::Rc;

use crate::class_driver::mouse::driver::MouseDriver;

use super::device_manager::device::device_map::DeviceMap;
use super::device_manager::DeviceManager;
use super::doorbell::DoorbellExternalRegisters;
use super::external_reg::ExternalRegisters;
use super::allocator::memory_allocatable::MemoryAllocatable;
use super::port::PortExternalRegisterss;
use super::scratchpad_buffers_array_ptr::ScratchpadBuffersArrayPtr;

pub trait DeviceContextOperations {
    fn write_device_context_array_addr(&mut self, device_context_addr: u64);

    fn setup_device_context_array(
        &mut self,
        device_slots: u8,
        scratchpad_buffers_len: usize,
        allocator: &mut impl MemoryAllocatable,
    ) -> DeviceContextArrayPtr  {
        let device_context_array_addr = allocator.try_allocate_device_context_array(device_slots + 1);

        let mut device_context_array = DeviceContextArrayPtr::new(device_context_array_addr);

        if 0 < scratchpad_buffers_len {
            let scratchpad_buffers_array = ScratchpadBuffersArrayPtr::new(scratchpad_buffers_len, allocator);
            device_context_array.set_device_context_at(0, scratchpad_buffers_array.base_addr());
        }

        self.write_device_context_array_addr(device_context_array_addr);

        device_context_array
    }
}

impl<M> DeviceContextOperations for ExternalRegisters<M>
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

pub(crate) fn setup_device_manager<T, M> (
    registers: &mut Rc<RefCell<T>>,
    device_slots: u8,
    scratchpad_buffers_len: usize,
    allocator: &mut impl MemoryAllocatable,
    mouse: MouseDriver, 
) -> DeviceManager<T, M>
where
    M: MemoryAllocatable,
    T: DeviceContextOperations
        + DoorbellExternalRegisters
        + PortExternalRegisterss
        + 'static,
{
    let device_context_array = registers.borrow_mut().setup_device_context_array(device_slots, scratchpad_buffers_len, allocator);

    DeviceManager::new(
        DeviceMap::default(),
        device_context_array,
        registers,
        mouse,
    )
}