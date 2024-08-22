use alloc::boxed::Box;

use crate::class_driver::{mouse::driver::MouseDriver, ClassDriverOperate};

use super::structs::{endpoint_descriptor::EndpointDescriptor, interface_descriptor::InterfaceDescriptor};
use crate::xhc::device_manager::endpoint_config::EndpointConfig;

pub struct HidDeviceDescriptors {
    interface: InterfaceDescriptor,
    endpoint: EndpointDescriptor,
}

impl HidDeviceDescriptors {
    pub fn new(interface: InterfaceDescriptor, endpoint: EndpointDescriptor) -> Self {
        Self {
            interface,
            endpoint,
        }
    }

    pub fn class_driver(
        &self,
        mouse: &MouseDriver
    ) -> Option<Box<dyn ClassDriverOperate>> {
        if self.interface.is_mouse() {
            return Some(Box::new(mouse.clone()));
        }

        None
    }

    pub fn interface(&self) -> InterfaceDescriptor {
        self.interface.clone()
    }


    pub fn endpoint_config(&self) -> EndpointConfig {
        EndpointConfig::new(&self.endpoint)
    }
}