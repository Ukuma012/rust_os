use volatile_bits::volatile_bits;
use crate::xhc::transfer::event::event_ring_segment_table::SegmentTableAddr;

#[volatile_bits(
    type=u32,
    add=0x08,
    bits=16
)]
pub struct RingSegmentSize(SegmentTableAddr);