use alloc::rc::Rc;
use core::cell::RefCell;
use super::{allocator::memory_allocatable::MemoryAllocatable, device_context::DeviceContextArrayPtr, doorbell::DoorbellExternalRegisterss, port::PortExternalRegisterss};

mod device;
mod device_context;
mod input_context;
mod endpoint_id;
mod device_context_index;
mod control_pipe;

pub struct DeviceManager<Doorbell, Memory> {
    devices: DeviceMap<Doorbell, Memory>,
    device_context_array: DeviceContextArrayPtr,
    addressing_port_id: Option<u8>,
    registers: Rc<RefCell<Doorbell>>,
    mouse: MouseDriver,
}

impl<Doorbell, Memory> DeviceManager<Doorbell, Memory>
where 
    Doorbell: DoorbellExternalRegisterss + PortExternalRegisterss + 'static,
    Memory: MemoryAllocatable,
{

}