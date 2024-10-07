use crate::configuration_space::msi::msi_capability_register::structs::capability_id::CapabilityId;

#[derive(Debug, Clone)]
pub struct Control {
    capability_id: CapabilityId,
    next_cap_ptr: u8,
    msi_enable: bool,
    multiple_msg_capable: u8,
    multiple_msg_enable: u8,
    is_64bit_addr_capable: bool,
    per_vector_masking_capable: bool,
}

impl Control {
    pub fn capability_id(&self) -> CapabilityId {
        self.capability_id
    }

    pub fn next_cap_ptr(&self) -> u8 {
        self.next_cap_ptr
    }

    pub fn msi_enable(&self) -> bool {
        self.msi_enable
    }

    pub fn is_64bit_addr_capable(&self) -> bool {
        self.is_64bit_addr_capable
    }

    pub fn set_msi_enable(&mut self) {
        self.msi_enable = true;
    }

    pub fn clear_msi_enable(&mut self) {
        self.msi_enable = false;
    }

    pub fn multiple_msg_enable(&mut self) -> u8 {
        self.multiple_msg_enable
    }

    pub fn set_multiple_msg_enable(&mut self, multiple_msg_enable: u8) {
        self.multiple_msg_enable = multiple_msg_enable & 0b111;
    }

    pub fn multiple_msg_capable(&mut self) -> u8 {
        self.multiple_msg_capable
    }

    pub fn raw(&self) -> u32 {
        let left = |v: u32, shift: u32| v << shift;
        let left_u8 = |v: u8, shift: u32| left(v as u32, shift);
        left(
            self.per_vector_masking_capable
                .into_bit(),
            24,
        ) | left(
            self.is_64bit_addr_capable
                .into_bit(),
            23,
        ) | left_u8(self.multiple_msg_enable, 20)
            | left_u8(self.multiple_msg_capable, 17)
            | left(self.msi_enable.into_bit(), 16)
            | left_u8(self.next_cap_ptr, 8)
            | self.capability_id as u32
    }
}