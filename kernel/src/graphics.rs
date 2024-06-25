use core::ops::AddAssign;
use common::frame_buffer::FrameBufferConfig;

pub struct Vector2D<T> {
    pub x: T,
    pub y: T
}

impl<T> AddAssign for Vector2D<T>
where
    T: AddAssign
    {
        fn add_assign(&mut self, other: Vector2D<T>) {
            self.x.add_assign(other.x);
            self.y.add_assign(other.y);
        }
    }

pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
impl PixelColor {
    pub const WHITE: Self = Self {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
    };

    pub const BLACK: Self = Self {
        r: 0x00,
        g: 0x00,
        b: 0x00,
    };

    pub const GREEN: Self = Self {
        r: 0x00,
        g: 0xFF,
        b: 0x00,
    };
}

pub fn write_pixel(config: &FrameBufferConfig, x: u32, y: u32, c: &PixelColor) {
    let pixel_position = config.stride * y + x;
    let base: isize = (4 * pixel_position) as isize;

    unsafe {
        let p = config.frame_buffer.offset(base);
            *p.offset(0) = c.b;
            *p.offset(1) = c.g;
            *p.offset(2) = c.r;
    }
}

pub fn fill_rectangle(frame_buffer: &FrameBufferConfig, pos: Vector2D<u32>, size: Vector2D<u32>, color: &PixelColor) {
    for y in 0..size.y {
        for x in 0..size.x {
            write_pixel(frame_buffer, pos.x + x, pos.y + y, color);
        }
    }
}

pub fn draw_rectangle(frame_buffer: &FrameBufferConfig, pos: Vector2D<u32>, size: Vector2D<u32>, color: &PixelColor) {
    for x in 0..size.x {
        write_pixel(frame_buffer, pos.x + x, pos.y, color);
        write_pixel(frame_buffer, pos.x + x, pos.y + size.y, color);
    }

    for y in 1..size.y-1 {
        write_pixel(frame_buffer, pos.x, pos.y + y, color);
        write_pixel(frame_buffer, pos.x + size.x - 1, pos.y + y, color);
    }
}
