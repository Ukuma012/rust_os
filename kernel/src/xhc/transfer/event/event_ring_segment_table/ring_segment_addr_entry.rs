use volatile_bits::volatile_bits;
use crate::xhc::transfer::event::event_ring_segment_table::SegmentTableAddr;

#[volatile_bits(type=u64)]
pub struct EventRingAddressEntry(SegmentTableAddr);