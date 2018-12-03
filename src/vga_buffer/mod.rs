use core::fmt::{self, Write};
use core::slice;
use core::sync::atomic::{AtomicUsize, Ordering};

mod color_code;
use self::color_code::{Color, ColorCode};

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut _;
const SCREEN_HEIGHT: usize = 25;
const SCREEN_WIDTH: usize = 80;

static X_POS: AtomicUsize = AtomicUsize::new(0);
static Y_POS: AtomicUsize = AtomicUsize::new(0);
static COLOR: ColorCode = ColorCode::new(Color::White, Color::Black);

pub struct Writer;

type VGABuf = [[ScreenChar; SCREEN_WIDTH]];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

#[allow(dead_code)]
impl Writer {
    pub fn clear_screen(&mut self) {
        for row in 0..SCREEN_HEIGHT {
            self.clear_row(row);
        }

        X_POS.store(0, Ordering::Relaxed);
        Y_POS.store(0, Ordering::Relaxed);
    }

    pub fn write_byte(&mut self, b: u8) {
        match b {
            b'\n' => self.new_line(),
            b'\r' => self.carriage_return(),
            _ => {
                let vga_buffer = Self::vga_buffer();
                let x = X_POS.load(Ordering::SeqCst);
                let y = Y_POS.fetch_add(1, Ordering::SeqCst);

                vga_buffer[x][y] = ScreenChar {
                    ascii_char: b,
                    color_code: COLOR,
                };

                if y + 1 >= SCREEN_WIDTH {
                    self.new_line();
                }
            }
        }
    }

    fn vga_buffer() -> &'static mut VGABuf {
        unsafe {
            slice::from_raw_parts_mut(
                VGA_BUFFER as *mut [ScreenChar; SCREEN_WIDTH],
                SCREEN_WIDTH * SCREEN_HEIGHT,
            )
        }
    }

    fn carriage_return(&mut self) {
        Y_POS.store(0, Ordering::Relaxed);
    }

    fn new_line(&mut self) {
        let vga_buffer = Self::vga_buffer();
        let x = X_POS.load(Ordering::SeqCst);

        if x >= SCREEN_HEIGHT - 1 {
            // copy every row to the row above it
            for row in 1..SCREEN_HEIGHT {
                for col in 0..SCREEN_WIDTH {
                    let ch = vga_buffer[row][col];
                    vga_buffer[row - 1][col] = ch;
                }
            }

            // clear the last row
            self.clear_row(SCREEN_HEIGHT - 1);
        } else {
            X_POS.fetch_add(1, Ordering::Relaxed);
        }

        Y_POS.store(0, Ordering::Relaxed);
    }

    fn clear_row(&mut self, row: usize) {
        let vga_buffer = Self::vga_buffer();
        let blank = ScreenChar {
            ascii_char: 0,
            color_code: COLOR,
        };

        for col in 0..SCREEN_WIDTH {
            vga_buffer[row][col] = blank;
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            self.write_byte(b);
        }

        Ok(())
    }
}

/*
pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
*/

/*
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_write_byte() {
        let mut writer = construct_writer();
        writer.write_byte(b'X');
        writer.write_byte(b'Y');

    }

    fn construct_writer() -> Writer {
        use std::boxed::Box;

        let buffer = construct_buffer();
        Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Blue, Color::Magenta),
            buffer: Box::leak(Box::new(buffer)),
        }
    }

    fn construct_buffer() -> Buffer {
        use array_init::array_init;

        Buffer {
            chars: array_init(|_| array_init(|| Volatile::new(empty_char())))
        }
    }

    fn empty_char() -> ScreenChar {
        ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Green, Color::Brown),
        }
    }

}
*/
