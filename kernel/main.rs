#![no_std]
#![feature(lang_items)]
#[macro_use]
extern crate console;
extern crate rlibc;
extern crate multiboot;
extern crate mem;
extern crate devices;
use devices::SerialPort;
use mem::HEAP;
use core::fmt::Write;
use core::fmt;
use multiboot::*;
use console::WRITER;
#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt(fmt: fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
    println!("PANIC on {} line {} col {}",file,line,col);
    loop{}
}
#[no_mangle] pub extern "C" fn _Unwind_Resume() {} //TODO
#[no_mangle]
pub extern "C" fn kmain(loader_info: &LoaderInfo,cs: u32,ce:u32,bs:u32,be:u32) {
    WRITER.lock().clear();
    //SerialPort::init();
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
        unsafe{HEAP.lock().add_region((mmap[1].addr+be) as usize,(mmap[1].addr+mmap[1].length-be)as usize);}
    }
    println!("Free memory: 0x{:X}",mem-(be-cs));
    unsafe{
        let mut first=0;
        let mut founded;
        let search="RSD PTR ".as_bytes();
        for index in 0xe0000..0xfffff{
            let ch=*(index as *const u8);
            if search[first]==ch{
                first+=1;
                if first==search.len(){
                    founded=index-7;
                    print!("RSDP founded on 0x{:08X}. OEMID: ",founded);
                    for i in founded+9..founded+15{
                        let ch=*(i as *const u8);
                        print!("{}",ch as char);
                    }
                    println!("");
                    first=0;
                }
            }else{
                first=0;
            }
        }
    }
}
