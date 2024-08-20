use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use core::cell::RefCell;
use crate::xhc::device_manager::device::Device;

pub struct DeviceMap<Doorbell, Memory> {
    map: BTreeMap<u8, Device<Doorbell, Memory>>
}

#[derive(Debug, Copy, Clone)]
pub struct DeviceConfig {
    parent_hub_slot_id: u8,
    port_speed: u8,
    slot_id: u8,
}

impl DeviceConfig {
    pub const fn new(parent_hub_slot_id: u8, port_speed: u8, slot_id: u8) -> Self {
        Self {
            parent_hub_slot_id,
            port_speed,
            slot_id,
        }
    }

    pub const fn parent_hub_slot_id(&self) -> u8 {
        self.parent_hub_slot_id
    }

    pub const fn port_speed(&self) -> u8 {
        self.port_speed
    }

    pub const fn slot_id(&self) -> u8 {
        self.slot_id
    }
}