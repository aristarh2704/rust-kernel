#![no_std]
#![feature(const_fn)]

extern crate multiboot;
#[macro_use]
extern crate devices;
extern crate mem;
use mem::Owned;
mod font;
use font::FontData;
use multiboot::FrameBuffer as mb_fb;

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
    backend: Owned<TtyBackend>
}
impl Tty{
    pub fn write_byte(&mut self,byte: char) {
        debug!("{}",byte as char);
        match byte {
            '\n'=>{
                self.row+=1;
                self.col=0;
                debug!("\r");
            }
            byte=>{
                self.backend.write_byte(self.row,self.col,byte as u32);
                self.col+=1;
                if self.col==self.max_col{
                    self.row+=1;
                    self.col=0;
                    debug!("\n\r");
                }
            }
        }
        if self.row==self.max_row{
            self.row-=1;
            self.backend.up();
        }
    }
    pub fn init(backend: Owned<TtyBackend>)->Self{
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
        for i in s.chars(){
            self.write_byte(i);
        }
        Ok(())
    }
}

pub trait TtyBackend{
    fn write_byte(&mut self,row: u32,col: u32,byte: u32); // TODO: Add Unicode support
    fn up(&mut self);
    fn get_size(&self)->(u32,u32); // (rows,cols)
}
impl TtyBackend{
    pub fn init(fb: &mb_fb)->Owned<TtyBackend>{
        if fb.flag==1{
            Framebuffer::init(fb)
        }else{
            Owned::new(Console::init())
        }
    }
}

struct Framebuffer{
    width: u32, // Количество пиксел в строке
    height: u32, // Количество пиксел в столбце
    bpp: u8, // Количество байт на символ
    font: Font,
    addr: u32 // Пока вот так...
}
impl TtyBackend for Framebuffer{
    fn write_byte(&mut self,row: u32,col: u32,byte: u32){
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
    pub fn init(fb: &mb_fb)->Owned<Framebuffer>{
        Owned::new(Framebuffer{
            width: fb.width,
            height: fb.height,
            bpp: fb.bpp/8,
            font: Font::init(),
            addr: fb.addr
        })
    }
}
struct Font{
    glyphs: &'static [u8],
    unicode: Owned<[u32]>,
    height:u32,
    width: u32,
    count: u32,
    flag: u8
}
impl Font{
    fn init()->Self{
        let font_data=FontData;
        let count=u32_from_u8(&font_data[16..20]);
        let unicode=Owned::new_array(0); //TODO
        //let flag=u32_from_u8(&font_data[12..16]);
        let height=u32_from_u8(&font_data[24..28]);
        let width=u32_from_u8(&font_data[28..32]);
        let glyphs=&font_data[32 as usize..((height*width*count/8)+32) as usize];
        Self{
            glyphs: glyphs,
            unicode: unicode,
            height: height,
            width: width,
            count: count,
            flag: 0
        }
    }
    fn get_symbol(&self,byte:u32)->&'static [u8]{
        &self.glyphs[16*byte as usize..(16*byte+16)as usize]
        //&self.data[addr..addr+size]
    }
}

struct Console{}
impl Console{
    fn init()->Console{
        let addr=0xb8000 as *mut u8;
        let attribute=ColorCode::new(Color::White,Color::Black).0;
        unsafe{
            for i in 0..80*25{
                *addr.offset(i*2 as isize)=0;
                *addr.offset((i*2+1) as isize)=attribute;
            }
        }
        Console{}
    }
}
impl TtyBackend for Console{
    fn write_byte(&mut self,row: u32,col: u32,byte: u32){
        unsafe{
            let attribute=ColorCode::new(Color::White,Color::Black).0;
            let offset=(row*80+col)*2;
            *((0xb8000 as *mut u8).offset(offset as isize))=byte as u8;
            *((0xb8000 as *mut u8).offset((offset+1) as isize))=attribute;
        }
    }
    fn up(&mut self){}
    fn get_size(&self)->(u32,u32){
        (25,80)
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

fn u32_from_u8(addr: &[u8])->u32{
    if addr.len()>=3{
        let mut result=0;
        for i in 0..4{
            result=(result<<8)+(addr[3-i] as u32);
        }
        result
    }else{0}
}