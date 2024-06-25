#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum PixelFormat {
    Rgb,
    Bgr,
}

#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct FrameBufferConfig {
    pub frame_buffer: *mut u8,
    pub stride: u32,
    pub resolution: (u32, u32), // (horizontal, vertical)
    pub format: PixelFormat,
}

impl FrameBufferConfig {
    pub fn width(&self) -> u32 {
        self.resolution.0
    }

    pub fn height(&self) -> u32 {
        self.resolution.1
    }
}