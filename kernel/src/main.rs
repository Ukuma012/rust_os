#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod graphics;
pub mod console;
pub mod frame_buffer;
pub mod interrupts;
pub mod gdt;
mod pci;
mod error;
mod usb;

use core::{panic::PanicInfo, arch::asm};
use common::frame_buffer::FrameBufferConfig;
use common::memory_map::MemoryMap;
use console::console_global;
use graphics::{graphics_global::{self, pixel_writer}, PixelColor};
use pci::scan_all_bus;
use usb::xhci::{mapper::IdentityMapper, xhci::XhciController, xhciregisters::XhciRegisters};

const K_MOUSE_CURSOR_WIDTH: usize = 15;
const K_MOUSE_CURSOR_HEIGHT: usize = 24;

const MOUSE_CURSOR_SHAPE: [[char; K_MOUSE_CURSOR_WIDTH]; K_MOUSE_CURSOR_HEIGHT] = [
    ['@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '.', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '.', '.', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '.', '.', '.', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' '],
    ['@', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '@', ' ', ' ', ' '],
    ['@', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '@', ' ', ' '],
    ['@', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '@', ' '],
    ['@', '.', '.', '.', '.', '@', '@', '@', '@', '@', '@', '@', '@', '@', '@'],
    ['@', '.', '.', '.', '.', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '.', '.', '@', '@', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '.', '@', ' ', '@', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '@', ' ', ' ', '@', '.', '@', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '.', '@', ' ', ' ', ' ', '@', '.', '@', ' ', ' ', ' ', ' ', ' ', ' '],
    ['@', '@', ' ', ' ', ' ', ' ', ' ', '@', '.', '@', ' ', ' ', ' ', ' ', ' '],
    ['@', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@', '.', '@', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@', '.', '@', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '@', '@', '@', ' ', ' ', ' '],
];

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer: &FrameBufferConfig, memory_map: &MemoryMap) {

    init(frame_buffer);
    pixel_writer().as_mut().unwrap().draw_desktop(frame_buffer.width(), frame_buffer.height());
    println!("{}", "Hello World!");

    let descriptors = memory_map.descriptors();
    descriptors.iter().for_each(|descriptor| {
         println!("{}, {}, {}, {}, {}", descriptor.memory_type, descriptor.phys_start, descriptor.phys_end, descriptor.virt_start, descriptor.att);
    });

    println!("{}", "It didn't crash!");

    for y in 0..K_MOUSE_CURSOR_HEIGHT {
        for x in 0..K_MOUSE_CURSOR_WIDTH {
            if MOUSE_CURSOR_SHAPE[y][x] == '@' {
                pixel_writer().as_mut().unwrap().write_pixel(200+x as u32, 100+y as u32, &PixelColor::WHITE);
            } else if MOUSE_CURSOR_SHAPE[y][x] == '.' {
                pixel_writer().as_mut().unwrap().write_pixel(200+x as u32, 100+y as u32, &PixelColor::BLACK);
            }
        }
    }

    let _all_buses = scan_all_bus().unwrap();
    let num_devices = pci::NUM_DEVICE.lock();
    let devices = pci::DEVICES.lock();
    let mut xhc_dev: Option<pci::Device> = None;
    for i in 0..*num_devices {
           if devices[i].class_code.is_match_all(0x0c, 0x03, 0x30) {
               xhc_dev = Some(devices[i]);

               // Prioritize Intel Products
               if 0x8086 == xhc_dev.as_ref().unwrap().vender_id() {
                   break;
               }
           }
    }

    if let Some(dev) = xhc_dev {
       let xhc_bar = pci::read_bar(&dev, 0).unwrap();
       let xhc_mmio_base = xhc_bar & !(0x0f as u64);

       let registers = XhciRegisters::new(xhc_mmio_base, IdentityMapper);

       let _xhc_controller = XhciController::new(registers);
       
    } else {
       println!("xHCI Device not found");
    }

    loop {
        unsafe {asm!("hlt")}
    }

}

fn init(config: &FrameBufferConfig) {
    graphics_global::init(*config);
    console_global::init();
    gdt::init();
    interrupts::init_idt();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        unsafe {asm!("hlt")}
    }
}
