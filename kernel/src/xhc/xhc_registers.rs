use super::capability_register::CapabilityExternalRegisterss;
use super::interrupter_set_register::InterrupterSetRegisterOperations;
use super::usb_command::UsbCommandRegisterOperations;
use super::doorbell::DoorbellExternalRegisterss;
use super::port::PortExternalRegisterss;
use super::config::ConfigRegisterOperations;
use super::device_context::DeviceContextOperations;
use super::registers_operation::RegistersOperation;

pub trait XhcRegisters:
    RegistersOperation
    + CapabilityExternalRegisterss
    + InterrupterSetRegisterOperations
    + UsbCommandRegisterOperations
    + DoorbellExternalRegisterss
    + PortExternalRegisterss
    + ConfigRegisterOperations
    + DeviceContextOperations
{
}