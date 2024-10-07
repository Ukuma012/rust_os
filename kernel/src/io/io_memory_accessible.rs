use core::fmt::{Debug, Formatter};
use crate::pci::read_data;
use crate::pci::write_address;
use crate::pci::IoIn32;
use crate::pci::IoOut32;

const CONFIG_DATA_REGISTER_ADDR: u16 = 0x0CFC;

#[derive(Debug, Clone)]
pub struct IoMemoryAccessor {}

impl IoMemoryAccessor {
    pub fn new() -> Self {
        Self{}
    }
}

impl IoMemoryAccessible for IoMemoryAccessor {
    fn io_in(&self, port: u16) -> u32 {
       unsafe {IoIn32(port)} 
    }

    fn io_out(&mut self, port: u16, value: u32) {
        unsafe {IoOut32(port, value)}
    }
}

pub trait IoMemoryAccessible {
    fn io_in(&self, port: u16) -> u32;

    fn io_out(&mut self, port: u16, value: u32);

    fn read_config_data_with_set_addr(&mut self, config_addr_register: ConfigAddrRegister) -> u32 {
        write_address(config_addr_register.as_data());
        read_data()
    }
}

pub struct ConfigAddrRegister {
    register_offset: u32,
    function: u32,
    device_slot: u32,
    bus: u32,
}

impl Debug for ConfigAddrRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "config_addr=0b{:b}", self.as_data())
    }
}

impl ConfigAddrRegister {
    pub fn new(register_offset: u8, function: u8, device_slot: u8, bus: u8) -> Self {
        Self {
            register_offset: register_offset as u32,
            function: function as u32,
            device_slot: device_slot as u32,
            bus: bus as u32,
        }
    }

    pub fn bus(&self) -> u32 {
        self.bus
    }
    pub fn device_slot(&self) -> u32 {
        self.device_slot
    }
    pub fn function(&self) -> u32 {
        self.function
    }
    pub fn register_offset(&self) -> u32 {
        self.register_offset
    }
    pub fn register_offset_with_mask(&self) -> u32 {
        // 下位2Bitは0である必要があるためビットマスク
        self.register_offset() & 0xFC
    }

    pub fn as_data(&self) -> u32 {
        let shift = |d: u32, shift_size: usize| (d << shift_size);

        shift(1, 31)
            | shift(self.bus(), 16)
            | shift(self.device_slot(), 11)
            | shift(self.function(), 8)
            | self.register_offset_with_mask()
    }
}
