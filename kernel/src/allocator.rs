use alloc::alloc::{GlobalAlloc, Layout};
use log::error;
use core::ptr::null_mut;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};

use crate::memory_manager::{frame_manager, Frame};

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub struct MemoryAllocator;

unsafe impl GlobalAlloc for MemoryAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let num_frames = (layout.size() + layout.align() + Frame::SIZE - 1) / Frame::SIZE;
        match  frame_manager().allocate(num_frames) {
            Ok(frame) => (frame.frame_id() * Frame::SIZE + layout.align()) as *mut u8,
            Err(_) => {
                null_mut()
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let frame_start = Frame::new(ptr as usize / Frame::SIZE);
        let num_frames = (layout.size() + layout.align() + Frame::SIZE - 1) / Frame::SIZE;
        frame_manager().free(frame_start, num_frames)
    }
}