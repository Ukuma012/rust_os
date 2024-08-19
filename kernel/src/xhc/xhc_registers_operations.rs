use super::capability_register::CapabilityRegistersOperations;
use super::interrupter_set_register::InterrupterSetRegisterOperations;
use super::usb_command::UsbCommandRegisterOperations;
use super::doorbell::DoorbellRegistersOperations;
use super::port::PortRegistersOperations;
use super::config::ConfigRegisterOperations;
use core::fmt::Debug;

pub trait XhcRegistersOperations:
    CapabilityRegistersOperations
    + InterrupterSetRegisterOperations
    + UsbCommandRegisterOperations
    + DoorbellRegistersOperations
    + PortRegistersOperations
    + ConfigRegisterOperations
    // + DeviceContextBaseAddressArrayPointerAccessible
{
}

pub struct RegistersOperation<M>(pub xhci::registers::Registers<M>)
where
    M: xhci::accessor::Mapper + Clone;

impl<M> RegistersOperation<M>
where
    M: xhci::accessor::Mapper + Clone,
 {
    pub fn new(mmio_base: u64, mapper: M) -> Self {
        let registers = unsafe { xhci::Registers::new(mmio_base.try_into().unwrap(), mapper) };

        Self(registers)
    }

    pub fn registers_mut(&mut self) -> &mut xhci::registers::Registers<M> {
        &mut self.0
    }

    pub fn reset(&mut self) {
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


impl<M> XhcRegistersOperations for RegistersOperation<M> where M: xhci::accessor::Mapper + Clone + Debug {}