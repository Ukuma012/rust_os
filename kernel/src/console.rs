use core::fmt;

use crate::graphics::{graphics_global::{self, pixel_writer}, write_ascii, PixelColor};

const ROWS: usize = 25;
const COLUMNS: usize = 80;

pub struct Console <'a>{
    fg_color: &'a PixelColor,
    bg_color: &'a PixelColor,
    cursor_row: usize,
    cursor_column: usize,
    buffer: [[char; COLUMNS + 1]; ROWS],
}

impl<'a> Console<'a> {
    pub fn new(fg_color: &'a PixelColor, bg_color: &'a PixelColor) -> Console<'a> {
        Self {
            fg_color,
            bg_color,
            cursor_row: 0,
            cursor_column: 0,
            buffer: [['\0'; COLUMNS + 1]; ROWS],
        }
    }

    pub fn put_string(&mut self, str: &str) {
        let mut writer_guard = graphics_global::pixel_writer();
        for char in str.chars() {
            if char == '\n' {
                self.new_line();
                continue;
            } else if self.cursor_column < COLUMNS - 1 {
                if let Some(writer) = writer_guard.as_mut() {
                    write_ascii(writer, 8 * self.cursor_column as u32, 16 * self.cursor_row as u32, char, &self.fg_color)
                }
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
            let mut writer_guard = graphics_global::pixel_writer();
            for row in 1..ROWS {
                self.buffer[row - 1] = self.buffer[row];
                for col in 0..COLUMNS {
                    let char = self.buffer[row - 1][col];
                    if let Some(writer) = writer_guard.as_mut() {
                        write_ascii(writer, 8 * col as u32, 16 * (row - 1) as u32, char, &self.fg_color);
                    }
                }
            }
            self.buffer[ROWS - 1] = [char::from(0); COLUMNS + 1];

            for col in 0..COLUMNS {
                if let Some(writer) = writer_guard.as_mut() {
                    write_ascii(writer, 8 * col as u32, 16 * (ROWS - 1) as u32, ' ', &self.fg_color)
                }
            }
        }
    }
}

pub mod console_global {
    use crate::graphics::PixelColor;
    use core::fmt;
    use core::fmt::Write;

    use super::Console;

    static mut CONSOLE: Option<Console<'static>> = None;

    pub fn init() -> () {
        unsafe {
            CONSOLE = Some(Console::new(&PixelColor::DESKTOP_FG, &PixelColor::DESKTOP_BG))
        };
    }

    pub fn console() -> &'static mut Console<'static> {
        unsafe {
            CONSOLE.as_mut().unwrap()
        }
    }

    pub fn _printk(args: fmt::Arguments) {
        console().write_fmt(args).unwrap();
    }
}

impl<'a> fmt::Write for Console<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.put_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => ($crate::console_global::_printk(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (printk!("\n"));
    ($($arg:tt)*) => (printk!("{}\n", format_args!($($arg)*)));
}