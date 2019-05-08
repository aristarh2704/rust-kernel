#![no_std]
extern crate io;
use io::*;

pub struct SerialPort {}
impl SerialPort {
    pub fn init() {
        unsafe {
            out_fn(0x3fa, 0);
            out_fn(0x3fb, 0x9b);
            out_fn(0x3f9, 0);
            out_fn(0x3f8, 0xc);
            out_fn(0x3fb, 0x1b);
        }
    }
    pub fn out_byte(byte: u8) {
        unsafe {
            out_fn(0x3f8, byte);
        }
    }
}
impl core::fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for i in s.as_bytes().iter() {
            SerialPort::out_byte(*i);
        }
        Ok(())
    }
}
/*
static mut buff:u32=0xb8000;
#[macro_export]
macro_rules! debug{
    ($($arg:tt)*) => ({use core::fmt::Write;let _=devices::EarlyConsole::new().write_fmt(format_args!($($arg)*));});
}
pub struct EarlyConsole{}
impl EarlyConsole{
    pub fn new()->EarlyConsole{
        EarlyConsole{}
    }
}
impl core::fmt::Write for EarlyConsole{
    fn write_str(&mut self,s: &str)->Result<(),core::fmt::Error>{
        for i in s.as_bytes(){
            unsafe{
                let x=buff as *mut u8;
                *x=*i;
                *x.offset(1)=0x0f;
                buff+=2;
            }
        }
        Ok(())
    }
}*/
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({use core::fmt::Write;devices::SerialPort{}.write_fmt(format_args!($($arg)*));});
}
