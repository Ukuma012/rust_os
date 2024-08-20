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

pub mod class_driver;
mod library;
mod graphics;
mod console;
mod frame_buffer;
mod interrupts;
mod gdt;
mod paging;
mod error;
mod memory_manager;
mod allocator;
mod pci;
mod xhc;

use core::{panic::PanicInfo, arch::asm};
use common::frame_buffer::FrameBufferConfig;
use common::memory_map::MemoryMap;
use graphics::pixel_writer;
use allocator::MemoryAllocator;
use pci::scan_all_bus;
use xhc::start_xhci_host_controller;

#[no_mangle]
pub extern "sysv64" fn kernel_stack_main(frame_buffer_config: &FrameBufferConfig, memory_map: &MemoryMap) {
   unsafe { init(frame_buffer_config, memory_map); }
    
    pixel_writer().as_mut().unwrap().draw_desktop(frame_buffer_config.width(), frame_buffer_config.height());

    println!("Hello World");

    let _all_buses = scan_all_bus().unwrap();
    let num_devices = pci::NUM_DEVICE.lock();
    let devices = pci::DEVICES.lock();
    let mut xhc_dev: Option<pci::Device> = None;
    //　PCIデバイスからxHCを探す
    for i in 0..*num_devices {
           if devices[i].class_code.is_match_all(0x0c, 0x03, 0x30) {
               xhc_dev = Some(devices[i]);

               // Prioritize Intel Products
               if 0x8086 == xhc_dev.as_ref().unwrap().vender_id() {
                   break;
               }
           }
    }

    // コントローラのリセット
    if let Some(dev) = xhc_dev {
       let xhc_bar = pci::read_bar(&dev, 0).unwrap();
       let xhc_mmio_base = xhc_bar & !(0x0f as u64);
       start_xhci_host_controller(xhc_mmio_base);

    } else {
       println!("xHCI Device not found");
    }

    loop {
        unsafe {asm!("hlt")}
    }
}

unsafe fn init(config: &FrameBufferConfig, memory_map: &MemoryMap) {
    graphics::init(*config);
    console::init();
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

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}