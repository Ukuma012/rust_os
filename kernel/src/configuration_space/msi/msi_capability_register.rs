use crate::{configuration_space::ConfigurationSpace, io::io_memory_accessible::IoMemoryAccessible};
use crate::configuration_space::msi::msi_capability_register::access::control::ControlAccessor;
use crate::configuration_space::msi::msi_capability_register::access::message_address::MessageAddressAccessor;
use crate::configuration_space::msi::msi_capability_register::access::message_data::MessageDataAccessor;

pub mod access;
pub mod structs;

#[derive(Clone)]
pub struct MsiCapabilityRegister<Io>
    where 
        Io: IoMemoryAccessible,
{
    control: ControlAccessor,
    message_address: MessageAddressAccessor,
    message_data: MessageDataAccessor,
    msi_cap_addr: u8,
    configuration_space: ConfigurationSpace,
    io: Io
}

impl<Io> MsiCapabilityRegister<Io>
    where 
        Io: IoMemoryAccessible,
{
    pub fn new(
        msi_cap_addr: u8,
        configuration_space: ConfigurationSpace,
        mut io: Io,
    ) -> MsiCapabilityRegister<Io> {
        let control = ControlAccessor::new();
        let message_address = MessageAddressAccessor::new(
        )
    }
}