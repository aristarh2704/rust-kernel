#![no_std]
#![feature(const_fn)]

extern crate multiboot;
extern crate devices;
mod font;
use font::FontData;
use multiboot::FrameBuffer as mb_fb;
/*
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
*/
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({use core::fmt::Write;devices::SerialPort{}.write_fmt(format_args!($($arg)*));});
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (let _=TTYMUTEX.get().write_fmt(format_args!($($arg)*)););
}
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub struct Tty{
    max_row: u32,
    max_col: u32,
    row:     u32,
    col:     u32,
    backend: &'static mut TtyBackend   
}

impl Tty{
    pub fn write_byte(&mut self,byte: u8) {
        match byte {
            b'\n'=>{
                self.row+=1;
                self.col=0;
            }
            byte=>{
                self.backend.write_byte(self.row,self.col,byte);
                self.col+=1;
                if self.col==self.max_col{
                    self.row+=1;
                    self.col=0;
                }
            }
        }
        if self.row==self.max_row{
            self.row-=1;
            self.backend.up();
        }
    }
    pub fn init(backend: &'static mut TtyBackend)->Self{
        let (rows,cols)=backend.get_size();
        Tty{
            max_row: rows,
            max_col: cols,
            row: 0,
            col: 0,
            backend: backend
        }
    }
}

impl core::fmt::Write for Tty{
    fn write_str(&mut self,s: &str)->Result<(),core::fmt::Error>{
        for i in s.as_bytes().iter(){
            self.write_byte(*i);
        }
        Ok(())
    }
}

pub trait TtyBackend{
    fn write_byte(&mut self,row: u32,col: u32,byte: u8); // TODO: Add Unicode support
    fn up(&mut self);
    fn get_size(&self)->(u32,u32); // (rows,cols)
}

pub struct Framebuffer{
    width: u32, // Количество пиксел в строке
    height: u32, // Количество пиксел в столбце
    bpp: u8, // Количество байт на символ
    font: Font,
    addr: u32 // Пока вот так...
}

impl TtyBackend for Framebuffer{
    fn write_byte(&mut self,row: u32,col: u32,byte: u8){
        let symbol=self.font.get_symbol(byte); // &[u8]
        let addr=self.addr as *mut u8;
        let height=self.font.height;
        let width=self.font.width; // Support only 8 bits width. TODO
        let mut y:u32=height*row;
        for i in 0..height{
            let mut bits=symbol[i as usize];
            let mut x=width*col+width-1;
            for i in 0..width{
                let off;
                if bits%2==1{
                    off=0xff;
                }else{
                    off=0x00;
                }
                bits/=2;
                let coord:isize=((y*self.width+x)*self.bpp as u32) as isize; // usize or isize? TODO
                for bpp in 0..self.bpp{
                    unsafe{
                        *addr.offset(coord+bpp as isize)=off;
                    }
                }
                x-=1;
            }
            y+=1;
        }
    }
    fn up(&mut self){}
    fn get_size(&self)->(u32,u32){
        let rows=self.height/self.font.height;
        let cols=self.width/self.font.width;
        (rows,cols)
    }
}

impl Framebuffer{
    pub fn init(fb: &mb_fb)->Framebuffer{
        Framebuffer{
            width: fb.width,
            height: fb.height,
            bpp: fb.bpp/8,
            font: Font::init(),
            addr: fb.addr
        }
    }
}

struct Font{
    data: &'static [u8],
    pub height:u32,
    pub width: u32
}

impl Font{
    fn init()->Self{
        Font{
            data: FontData,
            height: 16, // TODO: parsing font data.
            width: 8
        }
    }
    fn get_symbol(&self,byte:u8)->&'static [u8]{
        let size=(self.height*self.width/8) as usize;
        let addr=(byte as usize)*size +32;
        &self.data[addr..addr+size]
    }
}

unsafe impl core::marker::Sync for TtyMutex{}
pub struct TtyMutex{
    tty: *mut Tty
}
impl TtyMutex{
    pub fn get(&self)->&mut Tty{
        unsafe{
            &mut *(self.tty)
        }
    }
    const fn new()->Self{
        TtyMutex{
            tty: 0 as *mut Tty
        }
    }
    pub fn set(&self,tty:&mut Tty){
        let sf;
        let tty_static;
        unsafe{
            sf=&mut *(self as *const TtyMutex as *mut TtyMutex);
            tty_static=tty as *mut Tty;
        }
        sf.tty=tty_static;
    }
}
pub static TTYMUTEX:TtyMutex=TtyMutex::new();