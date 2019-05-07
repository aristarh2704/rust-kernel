#![no_std]
#![feature(lang_items)]
#![feature(panic_handler)]
#[macro_use]
extern crate console;
extern crate rlibc;
extern crate multiboot;
extern crate mem;
#[macro_use] // debug macros
extern crate devices;
use mem::HEAP;
use core::fmt::Write;
use core::fmt;
use multiboot::MultiBoot;
use console::*;
extern{
    static eh_frame: usize;
}
#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {debug!("eh_personality");loop{}}
#[panic_handler] #[no_mangle]  pub unsafe fn panicimpl(x:&core::panic::PanicInfo)->!{
    let mut ebp=(&x as *const _ as usize)-8;
    for i in 0..20{
        let eip=*((ebp+4) as *const usize);
        if eip==0{
            break;
        }
        ebp=*(ebp as *const usize);
        debug!("0x{:08X}",eip);
        debug!("\n");
    }
    debug!("Kernel panic: {}\n",x);
    loop{}
}
#[no_mangle] pub extern "C" fn _Unwind_Resume(){debug!("Unwind_Resume");loop{}} //TODO
#[no_mangle]
pub extern "C" fn kmain(loader_info: usize,cs: u32,be:u32) {
    let mb_info=multiboot::init(loader_info);
    mem::init(&mb_info.mmap,cs,be);
    console::init(&mb_info.fb);
    println!("Kernel loaded in this area: 0x{:08X} - 0x{:08X}",cs,be);
    let x=Searcher{
        start: 0xe0000,
        end: 0x7ffffff,
        size: 20,
        x: "RSD PTR "
    };
    for i in x{
        let rsdp=unsafe{&*(i.as_ptr() as *const RSDP)};
        if valid(rsdp){
            let rsdt=rsdp.rsdt;
            print_str(&rsdt.signature);
            println!("");
            let count=(rsdt.length-36)/4;
            let mut addr=Addr::new(rsdt as *const _ as usize);
            addr.read::<ACPISDTHeader>();
            for i in 0..count{
                let mut en=addr.read::<&mut ACPISDTHeader>();
                print!(" ");
                print_str(&en.signature);
                print!(" 0x{:08X}",*en as *const _ as u32);
                println!("");

                if i==0{
                    let mut addr=Addr::new(*en as *const _ as usize);
                    addr.read::<ACPISDTHeader>();
                    addr.read::<&ACPISDTHeader>();
                    let mut dsdt=addr.read::<&ACPISDTHeader>(); //&&DSDT
                    let ss=dsdt.length-36;
                    let mut addr=Addr::new(*dsdt as *const _ as usize);
                    addr.read::<ACPISDTHeader>();
                    println!("  DSDT size: {}",ss);
                    for i in 0..ss{
                        //print!("{}",*addr.read::<u8>() as char);
                    }
                    println!();
                }
                if i==2{
                    println!("  SSDT size: {}",en.length-36);
                }
            }
        }
    }
    print!("Done\n");
    //println!("Я думаю, ты слишком ленивый человек, чтобы создать нормальное ядро. Поэтому дальше запускаться не хочу, иди в жопу");
    println!("Eh_frame: 0x{:08X}",unsafe{&eh_frame} as *const _ as usize);
    panic!();
}

fn print_str(output:&[u8]){
    for i in 0..output.len(){
        print!("{}",output[i] as char);
    }
}

struct Addr{
    x: usize
}
impl Addr{
    fn new(x:usize)->Addr{
        Addr{
            x:x
        }
    }
    fn read<'a,'b,T>(&'a mut self)->&'b mut T{
        let addr=unsafe{
            &mut *(self.x as *mut T)
        };
        self.x+=core::mem::size_of::<T>();
        addr
    }
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
    rsdt: &'static ACPISDTHeader
}
#[repr(packed)]
struct RSDT{
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
    oemrevision: u32,
    creatorid: u32,
    creatorrevision: u32
}

fn valid<T>(addr: &T)->bool{
    let addr=unsafe{core::slice::from_raw_parts(addr as *const T as *const u8,core::mem::size_of::<T>())};
    let mut sum=0u8;
    for i in 0..addr.len(){
        sum+=addr[i];
    }
    sum==0
}
