#[repr(u8)]
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub enum CapabilityId {
    Msi = 0x5,
    MsiX = 0x11,
}

impl CapabilityId {
    pub fn try_from_u8(v: u8) -> Self {
        match v {
            0x05 => CapabilityId::Msi,
            0x11 => CapabilityId::MsiX,
            _ => panic!("CapabilityId illegal value = {}", v)
        }
    }
}