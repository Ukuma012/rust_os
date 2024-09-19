use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

use crate::{memory_manager::{frame_manager, Frame}, println};

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub struct MemoryAllocator;

unsafe impl GlobalAlloc for MemoryAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let number_of_frames = (layout.size() + layout.align() + Frame::SIZE - 1) / Frame::SIZE;
        match  frame_manager().allocate(number_of_frames) {
            Ok(frame) =>{
             let ptr = (frame.frame_id() * Frame::SIZE + layout.align()) as *mut u8;
             println!("Allocated {:?} at {:p} (frame_id: {})", layout, ptr, frame.frame_id());
             ptr
            },
            Err(_) => {
                println!("Failed to allocate {:?}", layout);
                null_mut()
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let frame_start = Frame::new(ptr as usize / Frame::SIZE);
        let number_of_frames = (layout.size() + layout.align() + Frame::SIZE - 1) / Frame::SIZE;
        println!("Deallocating {:?} at {:p} (frame_id: {})", layout, ptr, frame_start.frame_id());
        frame_manager().free(frame_start, number_of_frames)
    }
}