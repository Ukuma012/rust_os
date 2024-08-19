use super::xhc_registers_operations::RegistersOperation;

pub trait DoorbellRegistersOperations {
    fn notify_at(&mut self, index: usize, target: u8, stream_id: u16);
}

impl<M> DoorbellRegistersOperations for RegistersOperation<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    fn notify_at(&mut self, index: usize, target: u8, stream_id: u16) {
        self.registers_mut()
            .doorbell
            .update_volatile_at(index, |doorbell| {
                doorbell.set_doorbell_target(target);
                doorbell.set_doorbell_stream_id(stream_id);
            });
    }
}