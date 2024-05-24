use common::frame_buffer::FrameBuffer;

use crate::graphics::PixelColor;
use crate::font::write_ascii;

const ROWS: usize = 25;
const COLUMNS: usize = 80;

pub struct Console <'a>{
    frame_buffer: &'a FrameBuffer,
    fg_color: &'a PixelColor,
    bg_color: &'a PixelColor,
    cursor_row: usize,
    cursor_column: usize,
    buffer: [[char; COLUMNS + 1]; ROWS],
}

impl<'a> Console<'a> {
    pub fn new(fg_color: &'a PixelColor, bg_color: &'a PixelColor, frame_buffer: &'a FrameBuffer) -> Console<'a> {
        Self {
            frame_buffer,
            fg_color,
            bg_color,
            cursor_row: 0,
            cursor_column: 0,
            buffer: [['\0'; COLUMNS + 1]; ROWS],
        }
    }

    pub fn put_string(&mut self, str: &str) {
        for char in str.chars() {
            if char == '\n' {
                self.new_line();
                continue;
            } else if self.cursor_column < COLUMNS - 1 {
               write_ascii(&self.frame_buffer, 8 * self.cursor_column as u32, 16 * self.cursor_row as u32, char, &self.fg_color) 
            }
            self.buffer[self.cursor_row][self.cursor_column] = char;
            self.cursor_column += 1;
        }
    }

    fn new_line(&mut self) {
        self.cursor_column = 0;
        if self.cursor_row < ROWS - 1 {
            self.cursor_row += 1;
            return;
        } else {
            for row in 1..ROWS {
                self.buffer[row - 1] = self.buffer[row];
                for col in 0..COLUMNS {
                    let char = self.buffer[row - 1][col];
                    write_ascii(&self.frame_buffer, 8 * col as u32, 16 * (row - 1) as u32, char, &self.fg_color);
                }
            }
            self.buffer[ROWS - 1] = [char::from(0); COLUMNS + 1];

            for col in 0..COLUMNS {
                write_ascii(&self.frame_buffer, 8 * col as u32, 16 * (ROWS - 1) as u32, ' ', &self.fg_color);
            }

        }
    }

}