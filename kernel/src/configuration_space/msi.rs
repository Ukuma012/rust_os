#[derive(Debug)]
pub struct InterruptCapabilityResigerIter<Io>
    where
        Io: IoMemoryAccessible + Clone,
        {
            general_header: GeneralHeader,
            msi_cap_addr: u8,
            io: Io,
        }