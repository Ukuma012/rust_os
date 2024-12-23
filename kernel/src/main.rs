#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![feature(pointer_is_aligned_to)]
#![feature(strict_provenance)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

mod library;
mod graphics;
mod console;
mod frame_buffer;
mod interrupts;
mod gdt;
mod paging;
mod memory_manager;
mod allocator;
mod pci;
mod error;

use core::{panic::PanicInfo, arch::asm};
use common::frame_buffer::FrameBufferConfig;
use common::memory_map::MemoryMap;
use graphics::pixel_writer;
use allocator::MemoryAllocator;

#[no_mangle]
pub extern "sysv64" fn kernel_stack_main(frame_buffer_config: &FrameBufferConfig, memory_map: &MemoryMap) {
   unsafe { init(frame_buffer_config, memory_map); }
    
    pixel_writer().as_mut().unwrap().draw_desktop(frame_buffer_config.width(), frame_buffer_config.height());

    println!("Hello World");
    unsafe {
        paging::init();
        memory_manager::frame_manager().init(memory_map); // unsafe
    }
    
    loop {
        unsafe {asm!("hlt")}
    }
}

unsafe fn init(config: &FrameBufferConfig, _memory_map: &MemoryMap) {
    graphics::init(*config);
    console::init();
    gdt::init();
    interrupts::init();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        unsafe {asm!("hlt")}
    }
}

#[global_allocator]
static ALLOCATOR: MemoryAllocator = MemoryAllocator;

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> !{
    panic!("allocation error: {:?}", layout)
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}