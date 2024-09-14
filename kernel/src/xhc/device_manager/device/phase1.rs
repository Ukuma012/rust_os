use crate::{class_driver::mouse::driver::MouseDriver, println, xhc::{allocator::memory_allocatable::MemoryAllocatable, device_manager::{control_pipe::{request::Request, ControlPipeTransfer}, device::phase::InitStatus}, doorbell::DoorbellExternalRegisters}};
use alloc::boxed::Box;
use super::phase::Phase;
use super::phase2::Phase2;

pub struct Phase1 {
    mouse: MouseDriver,
}

impl Phase1 {
    pub const fn new(mouse: MouseDriver) -> Self {
        Self {
            mouse
        }
    }
}

impl<Doorbell, Memory> Phase<Doorbell, Memory> for Phase1
where 
    Memory: MemoryAllocatable,
    Doorbell: DoorbellExternalRegisters + 'static,
{
    fn on_transfer_event_received (
            &mut self,
            slot: &mut super::device_slot::DeviceSlot<Doorbell, Memory>,
            transfer_event: xhci::ring::trb::event::TransferEvent,
            target_event: crate::xhc::transfer::event::target_event::TargetEvent,
        ) -> (super::phase::InitStatus, Option<alloc::boxed::Box<dyn Phase<Doorbell, Memory>>>) {
        const CONFIGURATION_TYPE: u16 = 2;

        let data_buff_addr = slot.data_buff_addr();
        let len = slot.data_buff_len() as u32;
        let request = Request::get_descriptor(CONFIGURATION_TYPE, 0, len as u16);
        slot.default_control_pipe_mut()
            .control_in()
            .with_data(request, data_buff_addr, len);

        (InitStatus::not(), Some(Box::new(Phase2::new(self.mouse.clone()))))
    }

    fn interface_nums(&self) -> Option<alloc::vec::Vec<u8>> {
        None
    }
}