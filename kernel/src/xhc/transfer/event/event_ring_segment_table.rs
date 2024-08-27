use ring_segment_addr_entry::EventRingAddressEntry;
use ring_segment_size::RingSegmentSize;
use volatile_bits::{volatile_address, VolatileBitsWritable};

pub mod ring_segment_addr_entry;
pub mod ring_segment_size;

#[derive(Debug)]
pub struct EventRingSegmentTable {}

impl EventRingSegmentTable {
    pub fn new(
        event_ring_segment_table_addr: u64,
        event_ring_segment_addr: u64,
        ring_segment_size: usize,
    ) -> Self {
        let addr = SegmentTableAddr::from(event_ring_segment_table_addr);
        EventRingAddressEntry::from(addr)
            .write_volatile(event_ring_segment_addr);

        RingSegmentSize::from(addr)
            .write_volatile(ring_segment_size as u32);

        Self {}
    }
}

#[volatile_address]
pub struct SegmentTableAddr(u64);