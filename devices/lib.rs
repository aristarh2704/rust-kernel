#![no_std]
extern crate io;
use io::*;

pub struct SerialPort{}
impl SerialPort{
    pub fn init(){
        unsafe{
            out_fn(0x3fa,0);
            out_fn(0x3fb,0x9b);
            out_fn(0x3f9,0);
            out_fn(0x3f8,0xc);
            out_fn(0x3fb,0x1b);
        }
    }
    pub fn out_byte(byte:u8){
        unsafe{
            out_fn(0x3f8,byte);
        }
    }
}