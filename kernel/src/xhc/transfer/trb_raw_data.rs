use modular_bitfield::bitfield;
use modular_bitfield::prelude::B6;
use crate::xhc::transfer::trb_buffer_from_address;

#[bitfield(bits = 128)]
#[derive(Copy, Clone)]
pub struct TrbTemplate {
    pub parameter: u64,
    pub status: u32,
    pub cycle_bit: bool,
    pub evaluate_next_trb: bool,
    #[allow(non_snake_case)]
    _reserve1: u8,
    pub trb_type: B6,
    pub control: u16,
}

impl TrbTemplate {
    pub fn from_addr(addr: u64) -> Self {
        unsafe {*(addr as *const Self)}
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct TrbRawData(u128);

impl TrbRawData {
    pub fn new_unchecked(trb_raw_data: u128) -> Self {
        Self(trb_raw_data)
    }

    pub fn template(&self) -> TrbTemplate {
        unsafe { *((&self.0) as *const u128).cast::<TrbTemplate>() }
    }

    pub fn from_addr(addr: u64) -> Self {
        let trb_raw_data = unsafe { *(addr as *const u128) };
        Self::new_unchecked(trb_raw_data)
    }

    pub fn into_u32_array(self) -> [u32; 4] {
        self.into()
    }

    pub fn buffer_mut(&mut self) -> &mut [u32] {
        trb_buffer_from_address(&mut self.0)
    }

    pub fn raw(&self) -> u128 {
        self.0
    }
}

impl Into<[u32; 4]> for TrbRawData {
    fn into(self) -> [u32; 4] {
        into_u32_array(self.0)
    }
}


impl From<[u32; 4]> for TrbRawData {
    fn from(value: [u32; 4]) -> Self {
        TrbRawData::new_unchecked(into_u128(value))
    }
}


fn into_u128(raw_data: [u32; 4]) -> u128 {
    let mask = |raw_data: u32, shift: u128| (raw_data as u128) << (32 * shift);

    mask(raw_data[0], 3) | mask(raw_data[1], 2) | mask(raw_data[2], 1) | mask(raw_data[3], 0)
}


fn into_u32_array(raw_data: u128) -> [u32; 4] {
    unsafe {
        let raw_data = *(&raw_data as *const u128);
        let mask = |shift: u128| ((raw_data >> (32 * shift)) & 0xFFFF_FFFF) as u32;

        [
            mask(0),
            mask(1),
            mask(2),
            mask(3),
        ]
    }
}