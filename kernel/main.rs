#![no_std]
#![feature(lang_items)]
extern crate console;
use console::Console;
use core::slice;
use core::fmt::Write;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {
    let mut buffer=Console::new(unsafe{
        slice::from_raw_parts_mut(0x0b8000 as *mut u16,2000)
    });
    buffer.clean();
    write!(buffer,"PANIC!!!!");
    loop{}
}

#[no_mangle]
pub extern "C" fn kmain(x:u32,y:u32) {
    let mut buffer=Console::new(unsafe{
        slice::from_raw_parts_mut(0x0b8000 as *mut u16,2000)
    });
    buffer.clean();
    write!(buffer,"{} {}\n",x,y);
    let mut mmap:&mut[u32];
    let words=(y-4)/4;
    unsafe{
        mmap=slice::from_raw_parts_mut(x as *mut u32,words as usize);
    };
    let structs=words/5;
    write!(buffer,"Structs: {}\n",structs);
    for i in 0..structs{
        write!(buffer,"Number: {}\n  Base addr: {} {}\n  Length: {} {}\n  Type: {}\n",
        i,
        mmap[(i*5) as usize],
        mmap[(i*5+1) as usize],
        mmap[(i*5+2) as usize],
        mmap[(i*5+3) as usize],
        mmap[(i*5+4) as usize]);
    };
}

