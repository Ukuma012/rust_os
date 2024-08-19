use super::external_reg::ExternalRegisters;

pub trait RegistersOperation {
    fn reset(&mut self);
    fn run(&mut self);
}

impl<M> RegistersOperation for ExternalRegisters<M>
where 
    M: xhci::accessor::Mapper + Clone,
{
    fn reset(&mut self) {
        let registers = self.registers_mut();
        registers
            .operational
            .usbcmd
            .update_volatile(|usb_cmd| {
                usb_cmd.clear_run_stop();
            });

        while !registers
            .operational
            .usbsts
            .read_volatile()
            .hc_halted()
        {}
        registers
            .operational
            .usbcmd
            .update_volatile(|usb_cmd| {
                usb_cmd.set_host_controller_reset();
            });
        while registers
            .operational
            .usbsts
            .read_volatile()
            .controller_not_ready()
        {}
    }

    fn run(&mut self) {
        self.0
            .operational
            .usbcmd
            .update_volatile(|u| {
                u.set_interrupter_enable();
            });

        self.0
            .interrupter_register_set
            .interrupter_mut(0)
            .imod
            .update_volatile(|u| {
                u.set_interrupt_moderation_interval(100);
            });

        self.0
            .operational
            .usbcmd
            .update_volatile(|u| {
                u.set_run_stop();
            });

        while self
            .0
            .operational
            .usbsts
            .read_volatile()
            .hc_halted()
        {}
    }
}