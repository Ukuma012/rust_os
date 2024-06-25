use common::frame_buffer::FrameBufferConfig;

use crate::graphics::FrameBufferWriter;

pub struct FrameBuffer {
    config: FrameBufferConfig,
    pub writer: FrameBufferWriter
}

impl FrameBuffer {
    pub fn new(config: FrameBufferConfig) -> Self {
        Self {
            config,
            writer: FrameBufferWriter::new(config),
        }
    }
}