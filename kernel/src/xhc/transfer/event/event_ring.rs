use alloc::rc::Rc;
use core::cell::RefCell;

use crate::println;
use crate::xhc::allocator::memory_allocatable::MemoryAllocatable;
use crate::xhc::{interrupter_set_register::InterrupterSetRegisterOperations, transfer::transfer_ring::TransferRing};
use crate::xhc::transfer::trb_raw_data::TrbRawData;
use crate::xhc::transfer::event::event_trb::EventTrb;
use crate::xhc::transfer::trb_byte_size;

use super::event_ring_segment_table::EventRingSegmentTable;

pub struct EventRing<T> {
    transfer_ring: TransferRing,
    segment_base_addr: u64,
    interrupter_set: Rc<RefCell<T>>,
}

impl<T> EventRing<T>
where 
    T: InterrupterSetRegisterOperations,
{
    pub fn new(segment_base_addr: u64, ring_size: usize, interrupter_set: &Rc<RefCell<T>>) -> Self {
        Self {
            transfer_ring: TransferRing::new(
                interrupter_set.borrow_mut().read_dequeue_pointer_addr_at(0),
                ring_size,
                true,
            ),
            segment_base_addr,
            interrupter_set: Rc::clone(interrupter_set),
        }
    }

    pub fn has_front(&self) -> bool {
        if let Some(circle_bit) = self
            .read_event_trb()
            .and_then(|trb| trb.circle_bit())
        {
            circle_bit == self.transfer_ring.cycle_bit()
        } else {
            false
        }
    }

    pub fn read_event_trb(&self) -> Option<EventTrb> {
        let event_ring_dequeue_pointer_addr = self.read_dequeue_pointer_addr();

        let trb_raw_data =
            TrbRawData::new_unchecked(unsafe { *(event_ring_dequeue_pointer_addr as *mut u128) });

        EventTrb::new(trb_raw_data, self.transfer_ring.cycle_bit())
    }

    pub fn next_dequeue_pointer(&mut self) {
        let dequeue_pointer_addr = self.read_dequeue_pointer_addr();
        let next_addr = dequeue_pointer_addr + trb_byte_size();
        if self
            .transfer_ring
            .is_end_event_address(next_addr)
        {
            self.transfer_ring
                .toggle_cycle_bit();

            self.write_dequeue_pointer(self.segment_base_addr)
        } else {
            self.write_dequeue_pointer(next_addr)
        }
    }

    fn read_dequeue_pointer_addr(&self) -> u64 {
        self.interrupter_set
            .borrow_mut()
            .read_dequeue_pointer_addr_at(0)
    }


    fn write_dequeue_pointer(&mut self, addr: u64) {
        self.interrupter_set
            .borrow_mut()
            .update_dequeue_pointer_at(0, addr)
    }
}

pub(crate) fn setup_event_ring<T>(
    registers: &mut Rc<RefCell<T>>,
    event_ring_segment_table_size: u16,
    event_ring_segment_size: usize,
    allocator: &mut impl MemoryAllocatable,
) -> (EventRingSegmentTable, EventRing<T>)
where 
    T: InterrupterSetRegisterOperations,
{
    let event_ring_segment_table_addr =
        allocator.try_allocate_trb_ring(event_ring_segment_table_size as usize);

    let event_ring_segment_addr = allocator.try_allocate_trb_ring(event_ring_segment_size);

    registers
        .borrow_mut()
        .write_event_ring_segment_table_size(0, event_ring_segment_table_size);

    registers
        .borrow_mut()
        .write_event_ring_dequeue_pointer_at(0, event_ring_segment_addr);

    let event_ring_table = EventRingSegmentTable::new(
        event_ring_segment_table_addr,
        event_ring_segment_addr,
        event_ring_segment_size,
    );

    registers
        .borrow_mut()
        .write_event_ring_segment_table_pointer_at(0, event_ring_segment_table_addr);

    registers
        .borrow_mut()
        .write_interrupter_enable_at(0, true);

    let event_ring = EventRing::new(event_ring_segment_addr, 32, registers);

    (event_ring_table, event_ring)
}