use crate::xhc::xhc_registers_operations::RegistersOperation;

pub trait CapabilityRegistersOperations {
    fn read_max_scratchpad_buffers_len(&self) -> usize;
}

impl<M> CapabilityRegistersOperations for RegistersOperation<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn read_max_scratchpad_buffers_len(&self) -> usize {
        self.0
            .capability
            .hcsparams2
            .read_volatile()
            .max_scratchpad_buffers() as usize
    }
}