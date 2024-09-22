use shape::{draw_cursor, K_MOUSE_CURSOR_HEIGHT, K_MOUSE_CURSOR_WIDTH, MOUSE_CURSOR_SHAPE};

use crate::{graphics::{pixel_writer, PixelColor}, library::math::vector::Vector2D};

pub mod shape;

pub struct MouseCursor {
    pub erase_color: PixelColor,
    pub position: Vector2D<u32>
}

impl MouseCursor {
    pub fn new(
        initial_position: Vector2D<u32>
    ) -> Self {
       Self {
        erase_color: PixelColor::BLACK,
        position: initial_position,
       } 
    }

    pub fn draw(&self) {
        erase_mouse_cursor(self.position, &self.erase_color);
        draw_cursor(self.position);
    }

    pub fn move_relative(&mut self, displacement: &Vector2D<u32>) {
        erase_mouse_cursor(self.position, &self.erase_color);
        self.position += *displacement;
        draw_cursor(self.position);
    }
}

fn erase_mouse_cursor(
    position: Vector2D<u32>,
    erase_color: &PixelColor,
) {
    for y in 0..K_MOUSE_CURSOR_HEIGHT {
        for x in 0..K_MOUSE_CURSOR_WIDTH {
            if MOUSE_CURSOR_SHAPE[y][x] != ' ' {
                pixel_writer().as_mut().unwrap().write_pixel(position.x + x as u32, position.y + y as u32, &erase_color);
            }
        }
    }
}