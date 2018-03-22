#![no_std]
#![feature(const_fn)]
#![feature(unique)]
extern crate spin;
use core::fmt;
use core::ptr::Unique;
use spin::Mutex;
#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}
#[derive(Clone, Copy)]
pub struct ColorCode(u8);
impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
pub struct Writer {
    col: usize,
    row: usize,
    pub color_code: ColorCode,
    buffer: Unique<Buffer>,
}
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.row+=1;
                self.col=0;
            }
            byte => {
                let row = self.row;
                let col = self.col;
                self.buffer().chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };
                self.col +=1;
                if self.col == BUFFER_WIDTH {
                    self.col=0;
                    self.row+=1;
                }
            }
        }
        if self.row == BUFFER_HEIGHT {
            self.up();
        }
    }
    fn buffer(&mut self) -> &mut Buffer {
        unsafe{ self.buffer.as_mut() }
    }
    fn up(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][col];
                buffer.chars[row - 1][col]=character;
            }
        }
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[BUFFER_HEIGHT-1][col]=blank;
        }
        self.row-=1;
    }
    pub fn clear(&mut self){
        let character=ScreenChar{
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Red,Color::Black)
        };
        let buffer = self.buffer();
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                buffer.chars[row][col]=character;
            }
        }
    }
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
        Ok(())
    }
}
pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    col: 0,
    row: 0,
    color_code: ColorCode::new(Color::LightCyan, Color::Black),
    buffer: unsafe { Unique::new_unchecked(0xb8000 as *mut _) },
});
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({use core::fmt::Write;
    ::console::WRITER.lock().write_fmt(format_args!($($arg)*))})
}
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
