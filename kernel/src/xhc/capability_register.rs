use crate::xhc::external_reg::ExternalRegisters;

pub trait CapabilityExternalRegisterss {
    fn read_max_scratchpad_buffers_len(&self) -> usize;
}

impl<M> CapabilityExternalRegisterss for ExternalRegisters<M>
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