use volatile_bits::VolatileBitsReadable;

use crate::apic::LocalApicRegisters;

pub mod mouse;

pub fn enable_msi() {
    let bsp_local_apic_id: u8 = LocalApicRegisters::default().local_apic_id().read_volatile();
    unimplemented!()
}