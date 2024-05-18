use common::frame_buffer::FrameBuffer;

pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn write_pixel(config: &FrameBuffer, x: u32, y: u32, c: &PixelColor) {
    let pixel_position = config.stride * y + x;
    let base: isize = (4 * pixel_position) as isize;

    unsafe {
        let p = config.frame_buffer.offset(base);
            *p.offset(0) = c.r;
            *p.offset(1) = c.g;
            *p.offset(2) = c.b;
    }
}

