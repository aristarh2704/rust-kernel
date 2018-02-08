#![no_std]
#![feature(lang_items)]

extern crate console;
extern crate rlibc;
extern crate multiboot;

use core::fmt::Write;
use multiboot::*;
use console::writer;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {
    console::WRITER.lock().write_str("PANIC!!!");
    loop{}
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (WRITER.lock().write_fmt(format_args!($($arg)*)))
}
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[no_mangle]
pub extern "C" fn kmain(multiboot_addr: *const MultiBoot) {
    writer().clear();
    println!("Hello world");
    let mb_info=unsafe{
        &*multiboot_addr
    };
    print!("My loader is: ");
    println!("{}",unsafe{to_str(mb_info.loader)});
}

unsafe fn to_str(addr:*const u8)->&'static str{
    let mut index=0isize;
    loop{
        let byte=*addr.offset(index);
        if byte==0{
            break;
        }
        index+=1;
    }
    let slice=core::slice::from_raw_parts(addr,index as usize);
    core::str::from_utf8(slice).unwrap() // TODO
}
