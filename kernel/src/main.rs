#![no_std]
#![no_main]

pub mod graphics;
pub mod font;
pub mod console;
mod pci;

use core::{panic::PanicInfo, arch::asm};
use common::frame_buffer::FrameBuffer;
use common::memory_map::MemoryMap;
use console::Console;
use graphics::{draw_rectangle, fill_rectangle, Vector2D};
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

    let frame_width = frame_buffer.resolution.0;
    let frame_height = frame_buffer.resolution.1;

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

    fill_rectangle(frame_buffer, Vector2D { x: 0, y: 0 }, Vector2D { x: frame_width, y: frame_height }, &PixelColor {r: 30, g: 144, b: 255});
    fill_rectangle(frame_buffer, Vector2D { x: 0, y: frame_height - 50 }, Vector2D { x: frame_width, y: 50 }, &PixelColor { r: 1, g: 8, b: 17 });
    fill_rectangle(frame_buffer, Vector2D { x: 0, y: frame_height - 50 }, Vector2D { x: frame_width / 5, y: 50 }, &PixelColor { r: 80, g: 80, b: 80 });
    draw_rectangle(frame_buffer, Vector2D { x: 10, y: frame_height - 40 }, Vector2D { x: 30, y: 30 }, &PixelColor { r: 160, g: 160, b: 160 });

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