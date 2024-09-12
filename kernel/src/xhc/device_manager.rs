use alloc::rc::Rc;
use xhci::ring::trb::event::TransferEvent;
use crate::{println, xhc::device_manager::device::Device};
use core::cell::RefCell;
use super::{allocator::memory_allocatable::MemoryAllocatable, device_context::DeviceContextArrayPtr, doorbell::DoorbellExternalRegisters, port::PortExternalRegisterss, transfer::event::target_event::TargetEvent};
use crate::class_driver::mouse::driver::MouseDriver;
use super::device_manager::device::device_map::DeviceMap;
use crate::xhc::device_manager::device::device_map::DeviceConfig;

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
    pub fn new(
        devices: DeviceMap<Doorbell, Memory>,
        device_context_array: DeviceContextArrayPtr,
        registers: &Rc<RefCell<Doorbell>>,
        mouse: MouseDriver,
    ) -> Self {
        Self {
            devices,
            device_context_array,
            addressing_port_id: None,
            registers: Rc::clone(registers),
            mouse,
        }
    }

    pub fn device_slot_at(&mut self, slot_id: u8) -> &mut Device<Doorbell, Memory> {
        self.devices.get_mut(slot_id)
    }

    pub fn address_device(
        &mut self,
        slot_id: u8,
        allocator: &Rc<RefCell<Memory>>,
    ) -> u64 {
        let parent_hub_slot_id = self.try_addressing_port_id();

        let device = self.new_device(parent_hub_slot_id, slot_id, allocator);

        let device_context_addr = device.device_context_addr();
        let input_context_addr = device.input_context_addr();

        self.device_context_array.set_device_context_at(slot_id as usize, device_context_addr);

        self.addressing_port_id = None;

        input_context_addr
    }

    fn try_addressing_port_id(&self) -> u8 {
        self.addressing_port_id.expect("Not exists addressing port")
    }

    pub fn process_transfer_event(
        &mut self,
        slot_id: u8,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
    ) -> bool {
        let device = self.device_mut_at(slot_id);
        let init_status = device.on_transfer_event_received(transfer_event, target_event);

        init_status.is_initialized()
    }

    pub fn start_initialize_at(&mut self, slot_id: u8) {
        let device = self.devices.get_mut(slot_id);

        device.start_init()
    }

    pub fn configure_endpoint(&mut self, slot_id: u8) {
        let device = self.devices.get_mut(slot_id);

        device.on_endpoints_configured()
    }

    fn device_mut_at(&mut self, slot_id: u8) -> &mut Device<Doorbell, Memory> {
        self.devices.get_mut(slot_id)
    }

    fn new_device(
        &mut self,
        parent_hub_slot_id: u8,
        slot_id: u8,
        allocator: &Rc<RefCell<Memory>>
    ) -> &mut Device<Doorbell, Memory> {
        let port_speed = self.registers.borrow().read_port_spped_at(parent_hub_slot_id);

        let config = DeviceConfig::new(parent_hub_slot_id, port_speed, slot_id);

        self.devices.new_set(config, allocator, &self.registers, self.mouse.clone())
    }

    pub fn is_addressing_port(&self, port: u8) -> bool {
        if let Some(addressing_port) = self.addressing_port_id {
            port == addressing_port
        } else {
            true
        }
    }

    pub fn set_addressing_port_id(&mut self, port_id: u8) {
        self.addressing_port_id = Some(port_id);
    }
}