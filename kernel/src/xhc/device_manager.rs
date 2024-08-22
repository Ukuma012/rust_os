use alloc::rc::Rc;
use core::cell::RefCell;
use super::{allocator::memory_allocatable::MemoryAllocatable, device_context::DeviceContextArrayPtr, doorbell::DoorbellExternalRegisters, port::PortExternalRegisterss};
use crate::class_driver::mouse::driver::MouseDriver;
use super::device_manager::device::device_map::DeviceMap;

pub mod device;
pub mod device_context;
pub mod input_context;
pub mod endpoint_id;
pub mod device_context_index;
pub mod control_pipe;
pub mod descriptor;
pub mod endpoint_config;

pub struct DeviceManager<Doorbell, Memory> {
    devices: DeviceMap<Doorbell, Memory>,
    device_context_array: DeviceContextArrayPtr,
    addressing_port_id: Option<u8>,
    registers: Rc<RefCell<Doorbell>>,
    mouse: MouseDriver,
}

impl<Doorbell, Memory> DeviceManager<Doorbell, Memory>
where 
    Doorbell: DoorbellExternalRegisters + PortExternalRegisterss + 'static,
    Memory: MemoryAllocatable,
{

}