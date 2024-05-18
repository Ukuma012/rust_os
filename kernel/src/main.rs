#![no_std]
#![no_main]

pub mod graphics;
pub mod font;

use core::{panic::PanicInfo, arch::asm};
use common::frame_buffer::FrameBuffer;
use common::memory_map::MemoryMap;
use crate::graphics::{PixelColor, write_pixel};
use crate::font::write_ascii;

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer: &FrameBuffer, _memory_map: &MemoryMap) {

    let white = PixelColor {
        r: 255,
        g: 255,
        b: 255,
    };

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
            write_pixel(frame_buffer, x, y, &white);
        }
    }

    for x in 0..200 {
        for y in 0..100 {
            write_pixel(frame_buffer, 100+x, 100+y, &green);
        }
    }

    write_ascii(frame_buffer, 100, 50, 'A', &black);
    write_ascii(frame_buffer, 108, 50, 'A', &black);


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