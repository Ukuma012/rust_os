pub struct XhciRegisters<M>(xhci::registers::Registers<M>)
where
    M: xhci::accessor::Mapper + Clone;

impl<M> XhciRegisters<M>
where
    M: xhci::accessor::Mapper + Clone,
{
    pub fn new(mmio_base: u64, mapper: M) -> Self {
        let registers =
            unsafe { xhci::registers::Registers::new(mmio_base.try_into().unwrap(), mapper) };

        Self(registers)
    }

    pub fn reset(&mut self) -> () {
        let operational = &mut self.0.operational;

        operational.usbcmd.update_volatile(|usb_cmd| {
            usb_cmd.clear_run_stop();
        });

        while !operational.usbsts.read_volatile().hc_halted() {}

        operational.usbcmd.update_volatile(|usb_cmd| {
            usb_cmd.set_host_controller_reset();
        });

        while !operational.usbsts.read_volatile().controller_not_ready() {}
    }

    pub fn write_max_device_slots_enabled(&mut self, device_slots: u8) -> () {
        let operational = &mut self.0.operational;

        operational.config.update_volatile(|config| {
            config.set_max_device_slots_enabled(device_slots);
        });
    }
}
