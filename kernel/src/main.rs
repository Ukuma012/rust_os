#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
extern crate alloc;

mod graphics;
mod console;
mod frame_buffer;
mod interrupts;
mod gdt;
mod usb;
mod paging;
mod pci;
mod error;
mod memory_manager;
mod allocator;

use core::{panic::PanicInfo, arch::asm};
use alloc::boxed::Box;
use common::frame_buffer::FrameBufferConfig;
use common::memory_map::MemoryMap;
use console::console_global;
use graphics::graphics_global::{self, pixel_writer};
use allocator::MemoryAllocator;

#[no_mangle]
pub unsafe extern "sysv64" fn kernel_stack_main(frame_buffer_config: &FrameBufferConfig, memory_map: &MemoryMap) {
    init(frame_buffer_config, memory_map);
    
    pixel_writer().as_mut().unwrap().draw_desktop(frame_buffer_config.width(), frame_buffer_config.height());

    println!("{}", "Hello World!");

    alloc_test();

    println!("{}", "It didn't crash!");

    loop {
        unsafe {asm!("hlt")}
    }

}

fn alloc_test() {
    let x = Box::new(42);
    println!("{}", x);
}

unsafe fn init(config: &FrameBufferConfig, memory_map: &MemoryMap) {
    graphics_global::init(*config);
    console_global::init();
    gdt::init();
    interrupts::init();
    paging::init();
    memory_manager::frame_manager().init(memory_map); // unsafe
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
