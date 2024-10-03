use core::fmt::Debug;

use crate::io::io_memory_accessible::IoMemoryAccessible;
use crate::configuration_space::device::GeneralHeader;

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