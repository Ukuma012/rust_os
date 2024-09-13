use core::cell::RefCell;

use alloc::rc::Rc;

use crate::println;

use super::{allocator::memory_allocatable::MemoryAllocatable, doorbell::DoorbellExternalRegisters, external_reg::ExternalRegisters, transfer::command_ring::CommandRing};

pub trait UsbCommandRegisterOperations {
    fn write_command_ring_addr(&mut self, command_ring_addr: u64);
}

impl<M> UsbCommandRegisterOperations for ExternalRegisters<M>
where 
    M: xhci::accessor::Mapper + Clone,
{
    fn write_command_ring_addr(&mut self, command_ring_addr: u64) {
        let registers = self.registers_mut();

        registers
            .operational
            .crcr
            .update_volatile(|crcr| {
                crcr.set_ring_cycle_state();

                crcr.set_command_ring_pointer(command_ring_addr);
            });
    }
}

pub(crate) fn setup_command_ring<T>(
    registers: &mut Rc<RefCell<T>>,
    command_ring_size: usize,
    allocator: &mut impl MemoryAllocatable,
) -> CommandRing<T>
    where
        T: UsbCommandRegisterOperations + DoorbellExternalRegisters
    {
        let command_ring_addr = allocator.try_allocate_trb_ring(command_ring_size);
        let command_ring =
        CommandRing::new(command_ring_addr & !0b111111, command_ring_size, registers);
        registers
            .borrow_mut()
            .write_command_ring_addr(command_ring_addr);

        command_ring
    }