use core::{marker::PhantomData, ops::Range, ptr::read_volatile, ptr::write_volatile};
use crate::error::OsError;

#[derive(Clone, PartialEq, Eq)]
pub struct VendorDeviceId {
    pub vendor: u16,
    pub device: u16,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct BusDeviceFunction {
    id: u16,
}

struct ConfigRegisters<T> {
    access_type: PhantomData<T>,
}
impl<T> ConfigRegisters<T> {
    fn read(ecm_base: *mut T, byte_offset: usize) -> Result<T, OsError> {
        unsafe{
            Ok(read_volatile(ecm_base.add(byte_offset / size_of::<T>())))
        }
    }

    fn write(ecm_base: *mut T, byte_offset: usize, data: T) -> Result<(), OsError> {
        unsafe {
           write_volatile(ecm_base.add(byte_offset / size_of::<T>()), data)
        }

        Ok(())
    }
}

pub struct Pci {
    ecm_range: Range<usize>,
}

impl Pci {
    pub fn ecm_base<T>(&self, id: BusDeviceFunction) -> *mut T {
        (self.ecm_range.start + ((id.id as usize) << 12)) as *mut T
    }

    pub fn read_register_u8(&self, bdf: BusDeviceFunction, byte_offset: usize) -> Result<u8, OsError> {
        ConfigRegisters::read(self.ecm_base(bdf), byte_offset)
    }

    pub fn read_register_u16(&self, bdf: BusDeviceFunction, byte_offset: usize) -> Result<u16, OsError> {
        ConfigRegisters::read(self.ecm_base(bdf), byte_offset)
    }

    pub fn read_register_u32(&self, bdf: BusDeviceFunction, byte_offset: usize) -> Result<u32, OsError> {
        ConfigRegisters::read(self.ecm_base(bdf), byte_offset)
    }
    
    pub fn read_register_u64(&self, bdf: BusDeviceFunction, byte_offset: usize) -> Result<u64, OsError> {
        let lo = self.read_register_u32(bdf, byte_offset)?;
        let hi = self.read_register_u32(bdf, byte_offset + 4)?;
        Ok(((hi as u64) << 32) | (lo as u64))
    }

    pub fn write_register_u32(
        &self,
        bdf: BusDeviceFunction,
        byte_offset: usize,
        data: u32,
    ) -> Result<(), OsError> {
        ConfigRegisters::write(self.ecm_base(bdf), byte_offset, data)
    }
    pub fn write_register_u64(
        &self,
        bdf: BusDeviceFunction,
        byte_offset: usize,
        data: u64,
    ) -> Result<(), OsError> {
        let lo: u32 = data as u32;
        let hi: u32 = (data >> 32) as u32;
        self.write_register_u32(bdf, byte_offset, lo)?;
        self.write_register_u32(bdf, byte_offset + 4, hi)?;
        Ok(())
    }

    pub fn read_vendor_id_and_device_id(&self, id: BusDeviceFunction) -> Option<VendorDeviceId> {
        let vendor = self.read_register_u16(id, 0).ok()?;
        let device = self.read_register_u16(id, 2).ok()?;
        if vendor == 0xFFFF || device == 0xFFFF {
            None
        } else {
            Some(VendorDeviceId {vendor, device})
        }
    }
}