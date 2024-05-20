#![no_std]
#![no_main]

pub mod graphics;
pub mod font;
pub mod console;

use core::{panic::PanicInfo, arch::asm};
use common::frame_buffer::FrameBuffer;
use common::memory_map::MemoryMap;
use console::Console;
use crate::graphics::{PixelColor, write_pixel};

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer: &FrameBuffer, _memory_map: &MemoryMap) {

    let green = PixelColor {
        r: 51,
        g: 255,
        b: 51
    };

    let black = PixelColor {
        r: 0,
        g: 0,
        b: 0 
    };

    for x in 0..frame_buffer.resolution.0 {
        for y in 0..frame_buffer.resolution.1 {
            write_pixel(frame_buffer, x, y, &black);
        }
    }

    let mut console = Console::new(green, black, *frame_buffer);
    console.put_string("Hello\n");
    console.put_string("Hello\n");
    console.put_string("Hello World\n");

    loop {
        unsafe {asm!("hlt")}
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        unsafe {asm!("hlt")}
    }
}