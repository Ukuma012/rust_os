use super::xhc_registers_operations::RegistersOperation;

pub trait  ConfigRegisterOperations {
    fn write_max_device_slots_enabled(&mut self, max_device_slots: u8); 
}

impl<M> ConfigRegisterOperations for RegistersOperation<M>
where 
    M: xhci::accessor::Mapper + Clone,
{
    fn write_max_device_slots_enabled(&mut self, max_device_slots: u8) {
        self.registers_mut()
            .operational
            .config
            .update_volatile(|config| {
                config.set_max_device_slots_enabled(max_device_slots);
            });
    }
}