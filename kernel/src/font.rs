use common::frame_buffer::FrameBuffer;
use crate::graphics::{PixelColor, write_pixel};

const K_FONT_A: [u8; 16] = [
    0b00000000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00100100,
    0b00100100,
    0b00100100,
    0b00100100,
    0b01111110,
    0b01000010,
    0b01000010,
    0b01000010,
    0b11100111,
    0b00000000,
    0b00000000,
];

pub fn write_ascii(config: &FrameBuffer, x: u32, y: u32, c: char, color: &PixelColor) {
    if c != 'A' {
        return;
    }

    for dy in 0..16 {
        for dx in 0..8 {
            if (K_FONT_A[dy] << dx) & 0x80u8 != 0 {
                write_pixel(config, x+dx, y+dy as u32, color);
            }
        }
    }
}