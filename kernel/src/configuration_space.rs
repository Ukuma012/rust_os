use crate::{io::io_memory_accessible::ConfigAddrRegister, pci::{read_data, write_address}};

pub mod msi;
pub mod device;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct ConfigurationSpace {
    bus: u8,
    device_slot: u8,
    function: u8,
}

impl ConfigurationSpace {
    pub fn as_config_space(&self) -> &ConfigurationSpace {
        self
    }

    fn config_addr_at(&self, offset: u8) -> ConfigAddrRegister {
        ConfigAddrRegister::new(offset, self.function, self.device_slot, self.bus)
    }

    pub fn fetch_data_offset_at(&self, offset: u8) -> u32 {
        write_address(self.config_addr_at(offset).as_data());
        read_data()
    }
}