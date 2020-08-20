use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;
use vga::colors::{Color16, TextModeColor};
use vga::writers::{Screen, ScreenCharacter, Text80x25, TextWriter};

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = {
        let text_mode = Text80x25::new();
        text_mode.set_mode();
        text_mode.clear_screen();
        text_mode.enable_cursor();

        Mutex::new(Writer {
            row: 0,
            col: 0,
            color_code: TextModeColor::new(Color16::White, Color16::Black),
        })
    };
}

pub struct Writer {
    row: usize,
    col: usize,
    color_code: TextModeColor,
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

impl Writer {
    #[inline]
    fn write_byte(&mut self, byte: u8) {
        let text_mode = Text80x25::new();

        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= Text80x25::WIDTH {
                    self.new_line();
                }

                let screen_char = ScreenCharacter::new(byte, self.color_code);
                text_mode.write_character(self.col, self.row, screen_char);

                self.col += 1;
            }
        }
    }

    #[inline]
    fn write_string(&mut self, s: &str) {
        s.bytes().for_each(|b| self.write_byte(b));
    }

    #[inline]
    fn new_line(&mut self) {
        let text_mode = Text80x25::new();

        if self.row >= Text80x25::HEIGHT - 1 {
            // reached the bottom, time to scroll
            // copy every row to the row above
            for row in 1..Text80x25::HEIGHT {
                for col in 0..Text80x25::WIDTH {
                    let ch = text_mode.read_character(col, row);
                    text_mode.write_character(col, row + 1, ch);
                }
            }

            // clear last row
            self.clear_row(Text80x25::HEIGHT - 1);
        } else {
            self.row += 1;
        }

        self.col = 0;
    }

    #[inline]
    fn clear_row(&mut self, row: usize) {
        let text_mode = Text80x25::new();
        let blank = ScreenCharacter::new(b' ', self.color_code);

        for col in 0..Text80x25::WIDTH {
            text_mode.write_character(col, row, blank);
        }
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::kernel::devices::vga::_print(format_args!($($arg)*)));
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
