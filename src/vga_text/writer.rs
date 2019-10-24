// src/vga_text/writer.rs

use crate::vga_text::color_code::ColorCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    x: usize,
    y: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn clear_screen(&mut self) {
        let blank = ScreenChar {
            ascii_char: 0,
            color_code: self.color_code,
        };

        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row][col] = blank;
            }
        }

        self.x = 0;
        self.y = 0;
    }

    pub fn write_byte(&mut self, byte: u8) {
        //match byte {}
    }
}
