use crate::{graphics::PixelColor, library::math::vector::Vector2D};

pub mod shape;

pub struct MouseCursor {
    pub erase_color: PixelColor,
    pub initial_position: Vector2D<usize>
}

impl MouseCursor {
    pub fn new() -> Self {
       Self {
        erase_color: PixelColor::BLACK,
        initial_position: Vector2D { x: 300, y: 200 }
       } 
    }
}