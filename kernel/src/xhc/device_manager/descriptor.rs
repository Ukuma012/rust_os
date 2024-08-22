use structs::{configuration_descriptor::ConfigurationDescriptor, endpoint_descriptor::EndpointDescriptor, hid_descriptor::HidDescriptor, interface_descriptor::InterfaceDescriptor};

pub mod structs;
pub mod descriptor_sequence;
pub mod hid;

#[derive(Debug)]
pub enum Descriptor {
    Configuration(ConfigurationDescriptor),
    Interface(InterfaceDescriptor),
    Endpoint(EndpointDescriptor),
    Hid(HidDescriptor),
    NotSupport,
}

impl Descriptor {
    pub fn interface(&self) -> Option<&InterfaceDescriptor> {
        if let Self::Interface(interface) = self {
            Some(interface)
        } else {
            None
        }
    }
}