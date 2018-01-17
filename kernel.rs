#![no_std]
#![feature(lang_items)]
extern crate console;
use console::{clean,print_number,print};
use core::slice;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}

#[no_mangle]
pub extern "C" fn kmain() {
    /*let iter=IterateMaps{index:0};
    for my in iter{
        test(my);
    }*/
    let buffer=unsafe{
        slice::from_raw_parts_mut(0x0b8000 as *mut u16,2000)
    };
    clean(buffer);
    for x in 0..50{
        print_number(buffer,x);
    }
    print(buffer,"This is my own operating system\nvk.com/aristarh2704 \x01\x01".as_bytes())
}

