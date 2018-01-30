#![no_std]
#![feature(lang_items)]
extern crate console;
extern crate rlibc;
//extern crate mem;
use console::Console;
use core::slice;
use core::fmt::Write;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {
    let mut buffer=Console::new(unsafe{
        slice::from_raw_parts_mut(0x0b8000 as *mut u16,2000)
    });
    buffer.clean();
    let _result=write!(buffer,"PANIC!!!!");
    loop{}
}

#[no_mangle]
pub extern "C" fn kmain(x:u32,y:u32) {
    let mut buffer=Console::new(unsafe{
        slice::from_raw_parts_mut(0x0b8000 as *mut u16,2000)
    });
    buffer.clean();
    let mut _result=write!(buffer,"{} {}\n",x,y);
    let mmap:&mut[u32];
    let words=y/4;
    unsafe{
        mmap=slice::from_raw_parts_mut(x as *mut u32,words as usize);
    };
    let structs=words/6;
    _result=write!(buffer,"Structs: {}\n",structs);
    for i in 0..structs{
        _result=write!(buffer,"Number: {}\n  Base addr: 0x{:X}\n  Length: 0x{:X}\n  Type: {}\n",
        i,
        mmap[(i*6+1) as usize],
        mmap[(i*6+3) as usize],
        mmap[(i*6+5) as usize]);
    };
}

