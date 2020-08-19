// src/vga/writer.rs

use super::color_code::{Color, ColorCode};
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        row: 0,
        col: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    row: usize,
    col: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Writer {
    #[allow(dead_code)]
    #[inline]
    fn clear_screen(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }

        self.row = 0;
        self.col = 0;
    }

    #[inline]
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= BUFFER_WIDTH {
                    self.new_line();
                }

                let screen_char = ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code,
                };

                self.buffer.chars[self.row][self.col].write(screen_char);
                self.col += 1;
            }
        }
    }

    #[inline]
    fn write_string(&mut self, s: &str) {
        for b in s.bytes() {
            match b {
                // printable ASCII char
                0x20..=0x7e | b'\n' => self.write_byte(b),

                // not in ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    #[inline]
    fn new_line(&mut self) {
        if self.row >= BUFFER_HEIGHT - 1 {
            // reached the bottom, time to scroll
            // copy every row to the row above
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let ch = self.buffer.chars[row + 1][col].read();
                    self.buffer.chars[row][col].write(ch);
                }
            }

            // clear last row
            self.clear_row(BUFFER_HEIGHT - 1);
        } else {
            self.row += 1;
        }

        self.col = 0;
    }

    #[inline]
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::writer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
