extern crate x86;
use self::x86::io::outb;
pub struct SerialPort {}
const SER: SerialPort=SerialPort{};
impl SerialPort {
    pub fn init() {
        unsafe {
            outb(0x3fa, 0);
            outb(0x3fb, 0x9b);
            outb(0x3f9, 0);
            outb(0x3f8, 0xc);
            outb(0x3fb, 0x1b);
        }
    }
    pub fn out_byte(byte: u8) {
        unsafe {
            outb(0x3f8, byte);
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
pub fn init(){
	SerialPort::init();
}
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({use core::fmt::Write;crate::boot::x86::early_console::SerialPort{}.write_fmt(format_args!($($arg)*));});

}
