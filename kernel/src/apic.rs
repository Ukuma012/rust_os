use local_apic_id::LocalApicId;
use volatile_bits::volatile_address;

pub mod local_apic_id;

#[volatile_address]
pub struct LocalApicRegistersAddr(u64);


impl Default for LocalApicRegistersAddr {
    fn default() -> Self {
        LocalApicRegistersAddr::from(0xFEE00000)
    }
}

pub struct LocalApicRegisters {
    local_apic_id: LocalApicId
}

impl LocalApicRegisters {
    pub fn new(local_apic_addr: LocalApicRegistersAddr) -> Self {
        Self {
            local_apic_id: LocalApicId::new(local_apic_addr)
        }
    }

    pub fn local_apic_id(&self) -> &LocalApicId {
        &self.local_apic_id
    }
}

impl Default for LocalApicRegisters {
    fn default() -> Self {
        Self::new(LocalApicRegistersAddr::default())
    }
}