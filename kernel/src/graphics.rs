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

    pub const DESKTOP_BG: Self = Self {
        r: 45,
        g: 118,
        b: 237,
    };

    pub const DESKTOP_FG: Self = Self {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
    };
}

pub struct FrameBufferWriter {
    config: FrameBufferConfig,
}

impl FrameBufferWriter {
    pub fn new(config: FrameBufferConfig) -> Self {
        Self {
            config
        }
    }

    pub fn write_pixel(&self, x: u32, y: u32, c: &PixelColor) -> () {
        let pixel_position = self.config.stride * y + x;
        let base: isize = (4 * pixel_position) as isize;

        unsafe {
            let p = self.config.frame_buffer.offset(base);
                *p.offset(0) = c.b;
                *p.offset(1) = c.g;
                *p.offset(2) = c.r;
        }
    }

    pub fn fill_rectangle(&self, pos: Vector2D<u32>, size: Vector2D<u32>, c: &PixelColor) -> () {
        for y in 0..size.y {
            for x in 0..size.x {
                self.write_pixel(pos.x + x, pos.y + y, c)
            }
        }
    }

    pub fn draw_rectangle(&self, pos: Vector2D<u32>, size: Vector2D<u32>, c: &PixelColor) -> () {
        for x in 0..size.x {
            self.write_pixel(pos.x+x, pos.y, c);
            self.write_pixel(pos.x+x, pos.y+size.y, c);
        }

        for y in 0..size.y-1 {
            self.write_pixel(pos.x, pos.y+y, c);
            self.write_pixel(pos.x + size.x - 1, pos.y+y, c);
        }
    }

    pub fn draw_desktop(&self, width: u32, height: u32) {
        let green = PixelColor::GREEN;
        let white = PixelColor::WHITE;
        let black = PixelColor::BLACK;

        self.fill_rectangle(Vector2D { x: 0, y: 0 }, Vector2D { x: width, y: height }, &PixelColor {r: 30, g: 144, b: 255});
        self.fill_rectangle(Vector2D { x: 0, y: height - 50 }, Vector2D { x: width, y: 50 }, &PixelColor { r: 1, g: 8, b: 17 });
        self.fill_rectangle(Vector2D { x: 0, y: height - 50 }, Vector2D { x: width / 5, y: 50 }, &PixelColor { r: 80, g: 80, b: 80 });
        self.draw_rectangle(Vector2D { x: 10, y: height - 40 }, Vector2D { x: 30, y: 30 }, &PixelColor { r: 160, g: 160, b: 160 });
    }
}