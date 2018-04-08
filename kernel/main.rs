#![no_std]
#![feature(lang_items)]
#[macro_use]
extern crate console;
extern crate rlibc;
extern crate multiboot;
extern crate mem;
#[macro_use]
extern crate devices;
use devices::SerialPort;
use mem::{HEAP,Owned};
use core::fmt::Write;
use core::fmt;
use multiboot::{MultiBoot,LoaderInfo};
use console::*;

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt(fmt: fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
    println!("PANIC on {} line {} col {}",file,line,col);
    loop{}
}
#[no_mangle] pub extern "C" fn _Unwind_Resume() {} //TODO
#[no_mangle]
pub extern "C" fn kmain(loader_info: &LoaderInfo,cs: u32,ce:u32,bs:u32,be:u32) {
    let mut mb_info=MultiBoot::new();
    mb_info.init(loader_info);
    let mut mem=0;
    if let Some(mmap)=mb_info.mmap{
        for i in 0..mmap.len(){
            if mmap[i].flag==1{
                let mut reg_start=mmap[i].addr;
                let mut reg_end=mmap[i].addr+mmap[i].length;
                if reg_start==cs{
                    reg_start=be;
                }
                unsafe{HEAP.lock().add_region(reg_start as usize,(reg_end-reg_start) as usize)};
            }
        }
    }
    let mut my_backend=TtyBackend::init(&mb_info.get_fb());
    let mut tty=Tty::init(my_backend);
    TTYMUTEX.set(&mut tty);
    println!("Flags: {:013b}",mb_info.flags);
    println!("Kernel loaded in this area: 0x{:08X} - 0x{:08X}",cs,be);
    println!("Free memory: 0x{:X}",mem-(be-cs));
    let mut x=Searcher{
        start: 0xe0000,
        end: 0x7ffffff,
        size: 20,
        x: "RSD PTR "
    };
    for i in x{
        let mut checksum=0;
        for k in 0..20{
            checksum+=i[k];
        }
        if checksum==0{
            let mut addr=0u32;
            for k in 0..4{
                addr=(addr<<8)+i[19-k] as u32;
            }
            let rsdt=unsafe{&*(addr as *const Rsdt)};
            for k in 0..4{
                print!("{}",rsdt.header.signature[k] as char);
            }
        }
    }
    print!("Done\n");
    println!("Я думаю, ты слишком ленивый человек, чтобы создать нормальное ядро. Поэтому дальше запускаться не хочу, иди в жопу");
    panic!();
}

struct Searcher{
    start: u32,
    end: u32,
    size: u32,
    x: &'static str
}
impl Iterator for Searcher{
    type Item=&'static [u8];
    fn next(&mut self)->Option<Self::Item>{
        if self.start>self.end{
            return None
        }
        let arr=self.x.as_bytes();
        let mut first:u32=0;
        for index in self.start..self.end{
            let ch=unsafe{*(index as *const u8)};
            if arr[first as usize]==ch{
                first+=1;
                if first as usize==arr.len(){
                    self.start=index+1;
                    return Some(unsafe{core::slice::from_raw_parts((index-first+1)as *const u8,self.size as usize)})
                }
            }else{
                first=0;
            }
        }
        None
    }
}
#[repr(packed)]
struct RSDP{
    signature: [u8;8],
    checksum: u8,
    oemid: [u8;6],
    revision: u8,
    rsdt: &'static Rsdt
}
#[repr(packed)]
struct Rsdt{
    header: ACPISDTHeader
}
#[repr(packed)]
struct ACPISDTHeader{
    signature: [u8;4],
    length: u32,
    revision: u8,
    checksum: u8,
    oemid: [u8;6],
    oemtableid: [u8;8],
    creatorid: u32,
    creatorrevision: u32
}