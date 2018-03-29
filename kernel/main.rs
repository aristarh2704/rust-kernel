#![no_std]
#![feature(lang_items)]
#[macro_use]
extern crate console;
extern crate rlibc;
extern crate multiboot;
extern crate list;
use list::*;
use core::fmt::Write;
use multiboot::*;
use console::WRITER;
#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {
    print!("PANIC!!!");
    loop{}
}
#[no_mangle] pub extern "C" fn _Unwind_Resume() {} //TODO
#[no_mangle]
pub extern "C" fn kmain(loader_info: &LoaderInfo,cs: u32,ce:u32,bs:u32,be:u32) {
    WRITER.lock().clear();
    let mut mb_info=MultiBoot::new();
    mb_info.init(loader_info);
    println!("Flags: {:013b}",mb_info.flags);
    println!("Kernel loaded in this area: 0x{:08X} - 0x{:08X}",cs,be);
    println!("Kernel can use this areas:");
    let mut mem=0;
    if let Some(mmap)=mb_info.mmap{
        for i in 0..mmap.len(){
            if mmap[i].flag==1{
                println!("0x{:08X} - 0x{:08X}",mmap[i].addr,mmap[i].addr+mmap[i].length);
                mem+=mmap[i].length;
            }
        }
    }
    println!("Free memory: 0x{:X}",mem-(be-cs));
}
