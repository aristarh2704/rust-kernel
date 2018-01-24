#![no_std]
#![feature(lang_items)]
extern crate console;
use console::Console;
use core::slice;
use core::fmt::Write;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

#[no_mangle]
pub extern "C" fn kmain() {
    let mut buffer=Console::new(unsafe{
        slice::from_raw_parts_mut(0x0b8000 as *mut u16,2000)
    });
    buffer.clean();
    write!(buffer,"Hello, I am kernel))) My author's name is {}, he is {}","aristarh2704",17);
}

