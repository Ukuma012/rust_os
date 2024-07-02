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
    write_fn: fn(&Self, x: u32, y: u32, &PixelColor) -> (),
}

impl FrameBufferWriter {
    pub fn new(config: FrameBufferConfig) -> Self {
        Self {
            config,
            write_fn: Self::write_pixel
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
        self.fill_rectangle(Vector2D { x: 0, y: 0 }, Vector2D { x: width, y: height }, &PixelColor {r: 30, g: 144, b: 255});
        self.fill_rectangle(Vector2D { x: 0, y: height - 50 }, Vector2D { x: width, y: 50 }, &PixelColor { r: 1, g: 8, b: 17 });
        self.fill_rectangle(Vector2D { x: 0, y: height - 50 }, Vector2D { x: width / 5, y: 50 }, &PixelColor { r: 80, g: 80, b: 80 });
        self.draw_rectangle(Vector2D { x: 10, y: height - 40 }, Vector2D { x: 30, y: 30 }, &PixelColor { r: 160, g: 160, b: 160 });
    }

    pub fn write_ascii(&self, x: u32, y: u32, c: char, color: &PixelColor) -> () {
        let font = unsafe {get_font(c)};
        let font = match font {
            None => return,
            Some(f) => f,
        };

        for dy in 0..16 {
            for dx in 0..8 {
                let bits = unsafe {*font.offset(dy)};
                if (bits << dx) & 0x80 != 0 {
                    self.write_pixel(x+dx, y+dy as u32, color);
                }
            }
        }
    }
}

pub trait PixelWriter {
    fn write(&mut self, x: u32, y: u32, color: &PixelColor);
}

impl PixelWriter for FrameBufferWriter {
    fn write(&mut self, x: u32, y: u32, color: &PixelColor) {
        ((self.write_fn)(self, x, y, color));
    }
}

pub fn write_ascii<W: PixelWriter>(writer: &mut W, x: u32, y: u32, c: char, color: &PixelColor) -> () {
    let font = unsafe {get_font(c)};
    let font = match font {
        None => return,
        Some(f) => f,
    };

    for dy in 0..16 {
        for dx in 0..8 {
            let bits = unsafe {*font.offset(dy)};
            if (bits << dx) & 0x80 != 0 {
                writer.write(x+dx, y+dy as u32, color);
            }
        }
    } 
}

pub mod graphics_global {
    use common::frame_buffer::FrameBufferConfig;
    use spin::mutex::Mutex;
    use lazy_static::lazy_static;
    use super::FrameBufferWriter;

    lazy_static! {
        static ref FRAME_BUFFER_CONFIG: Mutex<Option<FrameBufferConfig>> = Mutex::new(None);
        static ref WRITER: Mutex<Option<FrameBufferWriter>> = Mutex::new(None);
    }


    pub fn init(config: FrameBufferConfig) {
        let mut fb_config = FRAME_BUFFER_CONFIG.lock();
        let mut writer = WRITER.lock();

        *fb_config = Some(config);
        *writer = Some(FrameBufferWriter::new(config));
    }

    pub fn frame_buffer_config() -> spin::MutexGuard<'static, Option<FrameBufferConfig>> {
        FRAME_BUFFER_CONFIG.lock()
    }

    pub fn pixel_writer() -> spin::MutexGuard<'static, Option<FrameBufferWriter>> {
        WRITER.lock()
    }
}

extern "C" {
    static _binary_hankaku_bin_start: u8;
    static _binary_hankaku_bin_end: u8;
    static _binary_hankaku_bin_size: u8;
}

unsafe fn get_font(c: char) -> Option<*mut u8> {
    let index = 16 * c as usize;
    let size = (&_binary_hankaku_bin_size as *const u8) as usize;

    if index < size {
        let start = (&_binary_hankaku_bin_start as *const u8) as *mut u8;
        Some(start.offset(index as isize))
    } else {
        None
    }
}