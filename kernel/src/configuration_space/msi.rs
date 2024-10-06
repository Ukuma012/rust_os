use core::fmt::Debug;

use crate::io::io_memory_accessible::IoMemoryAccessible;
use crate::configuration_space::device::GeneralHeader;

pub mod msi_capability_register;

#[derive(Debug)]
pub struct InterruptCapabilityResigerIter<Io>
    where
        Io: IoMemoryAccessible + Clone,
        {
            general_header: GeneralHeader,
            msi_cap_addr: u8,
            io: Io,
        }

impl <Io> InterruptCapabilityResigerIter<Io>
    where 
        Io: IoMemoryAccessible + Clone + Debug,
{
    pub fn new(general_header: GeneralHeader, io: Io) -> Self {
        let msi_cap_addr = general_header.msi_capability_pointer();
        Self {
            general_header,
            msi_cap_addr,
            io,
        }
    }
}

impl <Io> Iterator for InterruptCapabilityResigerIter<Io>
    where 
        Io: IoMemoryAccessible + Debug + Clone,
{
    type Item = InterruptCapabilityResiger<Io>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.msi_cap_addr == 0 {
            return None
        }

        let mut register = InterruptCapabilityResiger::new(
            self.general_header.clone(),
            self.msi_cap_addr,
            self.io.clone()
        );

        self.msi_cap_addr = register.as_mut().map_or(|r| {
            r.next_msi_cap_addr().unwrap_or(0)
        });

        Some(register)
    }
}

pub enum InterruptCapabilityResiger<Io>
    where 
        Io: IoMemoryAccessible + Clone,
{
    Msi(MsiCapabilityRegister<Io>),
    MsiX(MsiXCapabilityRegisters<Io>),
}