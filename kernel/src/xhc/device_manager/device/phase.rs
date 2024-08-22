use xhci::ring::trb::event::TransferEvent;
use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::xhc::transfer::event::target_event::TargetEvent;

use crate::xhc::{allocator::memory_allocatable::MemoryAllocatable, doorbell::DoorbellExternalRegisters};

use super::device_slot::DeviceSlot;

pub(crate) const DATA_BUFF_SIZE: usize = 256;

pub struct InitStatus(bool);

impl InitStatus {
    pub fn new(is_initialized: bool) -> Self {
        Self(is_initialized)
    }

    pub fn not() -> Self {
        Self::new(false)
    }

    pub fn initialized() -> Self {
        Self::new(true)
    }

    pub fn is_initialized(&self) -> bool {
        self.0
    }
}

pub trait Phase<Doorbell, Memory>
where
    Doorbell: DoorbellExternalRegisters,
    Memory: MemoryAllocatable,
{
    fn on_transfer_event_received (
        &mut self,
        slot: &mut DeviceSlot<Doorbell, Memory>,
        transfer_event: TransferEvent,
        target_event: TargetEvent,
    ) -> (InitStatus, Option<Box<dyn Phase<Doorbell, Memory>>>);

    fn interface_nums(&self) -> Option<Vec<u8>>;
}