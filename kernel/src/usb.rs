use volatile_bits::VolatileBitsReadable;

use crate::{apic::LocalApicRegisters, configuration_space::{device::GeneralHeader, msi::InterruptCapabilityResigerIter}, io::io_memory_accessible::IoMemoryAccessor};

pub mod mouse;

pub fn enable_msi(general_header: GeneralHeader) {
    let io = IoMemoryAccessor::new();
    let bsp_local_apic_id: u8 = LocalApicRegisters::default().local_apic_id().read_volatile();

    // for mut msi in InterruptCapabilityResigerIter::new(general_header, io)
    //     .filter_map()
    unimplemented!()
}