use alloc::boxed::Box;
use alloc::vec::Vec;

use xhci::ring::trb::event::TransferEvent;
use crate::class_driver::interrupt_in::InterruptIn;
use crate::class_driver::mouse::driver::MouseDriver;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::device_manager::descriptor::hid::HidDeviceDescriptors;
use crate::xhc::device_manager::device::device_slot::DeviceSlot;
use crate::xhc::device_manager::device::phase::{InitStatus, Phase};
use crate::xhc::device_manager::device_context_index::DeviceContextIndex;
use crate::xhc::transfer::event::target_event::TargetEvent;
use crate::xhc::doorbell::DoorbellExternalRegisters;

use super::phase4::Phase4;

pub struct Phase3 {
    mouse: MouseDriver,
    hid_device_descriptor_vec: Vec<HidDeviceDescriptors>,
}

impl Phase3 {
    pub const fn new(
        mouse: MouseDriver,
        hid_device_descriptor_vec: Vec<HidDeviceDescriptors>
    ) -> Self {
        Self {
            mouse,
            hid_device_descriptor_vec,
        }
    }

    fn interrupters<Doorbell, Memory>(
        &mut self,
        slot: &mut DeviceSlot<Doorbell, Memory>,
    ) -> Vec<InterruptIn<Doorbell>>
    where 
        Memory: MemoryAllocatable,
        Doorbell: DoorbellExternalRegisters,
    {
        self.hid_device_descriptor_vec
            .iter()
            .filter_map(|hid| {
                let class_driver = hid.class_driver(&self.mouse)?;
                let transfer_ring = slot.try_alloc_transfer_ring(32);
                Some(InterruptIn::new(
                    slot.id(),
                    class_driver,
                    &hid.endpoint_config(),
                    transfer_ring,
                    slot.doorbell(),
                    hid.interface(),
                ))
            }).collect()
    }
}

impl<Doorbell, Memory> Phase<Doorbell, Memory> for Phase3
where
    Memory: MemoryAllocatable,
    Doorbell: DoorbellExternalRegisters + 'static,
{
    fn on_transfer_event_received(
        &mut self,
        slot: &mut DeviceSlot<Doorbell, Memory>,
        _transfer_event: TransferEvent,
        _target_event: TargetEvent,
    ) -> (InitStatus, Option<Box<dyn Phase<Doorbell, Memory>>>) {
        slot.input_context_mut()
            .clear_control();

        slot.copy_device_context_to_input();

        slot.input_context_mut()
            .set_enable_slot_context();

        slot.input_context_mut()
            .slot_mut()
            .set_context_entries(31);

        let interrupters = self.interrupters(slot);
        interrupters
            .iter()
            .for_each(|interrupt| {
                let config = interrupt.endpoint_config();
                let dci = DeviceContextIndex::from_endpoint_id(config.endpoint_id());

                slot.input_context_mut()
                    .set_enable_endpoint(dci);

                let endpoint_ctx = slot
                    .input_context_mut()
                    .endpoint_mut_at(dci.value());

                config.write_endpoint_context(interrupt.transfer_ring_addr(), endpoint_ctx);
            });

        (
            InitStatus::initialized(),
            Some(Box::new(Phase4::new(interrupters))),
        )
    }

    fn interface_nums(&self) -> Option<Vec<u8>> {
        None
    }
}