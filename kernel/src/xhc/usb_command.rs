use super::xhc_registers_operations::RegistersOperation;

pub trait UsbCommandRegisterOperations {
    fn write_command_ring_addr(&mut self, command_ring_addr: u64);
}

impl<M> UsbCommandRegisterOperations for RegistersOperation<M>
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