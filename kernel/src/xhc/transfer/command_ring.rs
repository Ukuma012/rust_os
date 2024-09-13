use core::cell::RefCell;
use alloc::rc::Rc;

use xhci::ring::trb::command::ConfigureEndpoint;
use crate::println;
use crate::xhc::doorbell::DoorbellExternalRegisters;
use crate::xhc::transfer::transfer_ring::TransferRing;

pub struct CommandRing<T> {
    transfer_ring: TransferRing,
    doorbell: Rc<RefCell<T>>,
}

impl<T> CommandRing<T>
where 
    T: DoorbellExternalRegisters,
{
    pub fn new(ring_ptr_base_addr: u64, ring_size: usize, doorbell: &Rc<RefCell<T>>) -> Self {
        Self {
            transfer_ring: TransferRing::new(ring_ptr_base_addr, ring_size, true),
            doorbell: Rc::clone(doorbell),
        }
    }

    pub fn push_no_op(&mut self) {
        self.transfer_ring.push(xhci::ring::trb::command::Noop::new().into_raw());
        self.notify()
    }

    pub fn push_reset_endpoint(&mut self, slot_id: u8, endpoint_id: u8) {
        let mut reset_endpoint = xhci::ring::trb::command::ResetEndpoint::new();
        reset_endpoint.set_endpoint_id(endpoint_id);
        reset_endpoint.set_slot_id(slot_id);

        self.transfer_ring.push(reset_endpoint.into_raw());
        self.notify()
    }


    pub fn push_configure_endpoint(&mut self, input_context_addr: u64, slot_id: u8) {
        let mut configure_endpoint_trb = ConfigureEndpoint::new();
        configure_endpoint_trb.set_slot_id(slot_id);
        configure_endpoint_trb.set_input_context_pointer(input_context_addr);

        self.transfer_ring
            .push(configure_endpoint_trb.into_raw());
        self.notify()
    }


    pub fn push_address_command(&mut self, input_context_addr: u64, slot_id: u8) {
        let mut address_command = xhci::ring::trb::command::AddressDevice::new();
        address_command.set_input_context_pointer(input_context_addr);
        address_command.set_slot_id(slot_id);

        self.transfer_ring
            .push(address_command.into_raw());
        self.notify()
    }


    pub fn push_enable_slot(&mut self) {
        self.transfer_ring
            .push(xhci::ring::trb::command::EnableSlot::new().into_raw());
        self.notify()
    }

    fn notify(&mut self) {
        self.doorbell.borrow_mut().notify_at(0, 0, 0)
    }
}