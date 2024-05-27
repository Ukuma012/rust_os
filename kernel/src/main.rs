#![no_std]
#![no_main]

pub mod graphics;
pub mod font;
pub mod console;

use core::{panic::PanicInfo, arch::asm};
use common::frame_buffer::FrameBuffer;
use common::memory_map::MemoryMap;
use console::Console;
use graphics::{fill_rectangle, Vector2D};
use crate::graphics::{PixelColor, write_pixel};

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

    let white = PixelColor {
        r: 255,
        g: 255,
        b: 255 
    };

    for x in 0..frame_buffer.resolution.0 {
        for y in 0..frame_buffer.resolution.1 {
            write_pixel(frame_buffer, x, y, &black);
        }
    }

    let mut console = Console::new(&green, &black, &frame_buffer);
    console.put_string("Hello World\n");

    for y in 0..K_MOUSE_CURSOR_HEIGHT {
        for x in 0..K_MOUSE_CURSOR_WIDTH {
            if MOUSE_CURSOR_SHAPE[y][x] == '@' {
                write_pixel(frame_buffer, 200+x as u32, 100+y as u32, &white);
            } else if MOUSE_CURSOR_SHAPE[y][x] == '.' {
                write_pixel(frame_buffer, 200+x as u32, 100+y as u32, &black);
            }
        }
    }

    fill_rectangle(frame_buffer, Vector2D {x: 100, y: 100}, Vector2D {x: 300, y: 600}, &white);

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