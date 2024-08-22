use core::cell::RefCell;
use alloc::rc::Rc;
use xhci::ring::trb::transfer::{StatusStage, TransferType, Direction};
use crate::xhc::{device_manager::device_context_index::DeviceContextIndex, doorbell::DoorbellExternalRegisters, transfer::transfer_ring::TransferRing};
use crate::xhc::device_manager::control_pipe::{make_data_stage, make_setup_stage};
use super::ControlPipeTransfer;

pub struct ControlIn<T> {
    slot_id: u8,
    device_context_index: DeviceContextIndex,
    doorbell: Rc<RefCell<T>>,
    transfer_ring: Rc<RefCell<TransferRing>>,
}

impl<T> ControlIn<T>
where 
    T: DoorbellExternalRegisters,
{
    pub fn new(
        slot_id: u8,
        device_context_index: DeviceContextIndex,
        doorbell: &Rc<RefCell<T>>,
        transfer_ring: &Rc<RefCell<TransferRing>>,
    ) -> Self {
        Self {
            slot_id,
            device_context_index,
            doorbell: Rc::clone(doorbell),
            transfer_ring: Rc::clone(transfer_ring),
        }
    }

    pub fn notify(&mut self) {
        self.doorbell
            .borrow_mut()
            .notify_at(
                self.slot_id as usize,
               self.device_context_index.as_u8(),
               0
                    )
    }

    fn push(&mut self, trb_buff: [u32; 4]) {
        self.transfer_ring
            .borrow_mut()
            .push(trb_buff)
    }
}

impl<T> ControlPipeTransfer for ControlIn<T>
where 
    T: DoorbellExternalRegisters,
{
    fn no_data(&mut self, request: super::request::Request) {
        let setup_stage = make_setup_stage(request.setup_stage(), TransferType::No);
        self.push(setup_stage.into_raw());

        let mut status = StatusStage::new();
        status.set_direction();
        status.set_interrupt_on_completion();
        self.push(status.into_raw());
        self.notify()
    }

    fn with_data(&mut self, request: super::request::Request, data_buff_addr: u64, len: u32) {
        let setup = make_setup_stage(request.setup_stage(), TransferType::In);
        self.push(setup.into_raw());

        let mut data_stage = make_data_stage(data_buff_addr, len, Direction::In);
        data_stage.set_interrupt_on_completion();
        data_stage.set_interrupt_on_short_packet();

        self.push(data_stage.into_raw());

        self.push(StatusStage::new().into_raw());
        self.notify()
    }
}