#![no_std]
#![feature(const_fn)]
extern crate spin;
// Description of Multiboot information structure is in 
// https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Boot-information-format
mod boot;
pub use boot::MultiBoot as LoaderInfo;

pub struct MultiBoot{
    pub flags:  u32,
    pub mem:    Option<LUMem>,
    pub boot:   Option<BootDevice>,
    pub cmdline:Option<&'static str>,
    pub mods:   Option<Modules>,
    pub syms:   Option<SymbolTable>,
    pub mmap:   Option<&'static [boot::Frame]>,
    pub drives: Option<&'static Drive<u8>>,
    //pub config: &'static BIOSConfigTable,
    pub loader: Option<&'static str>,
    //pub apm:    &'static APMTable,
    //pub vbe:    VBE,
    pub fb:     Option<FrameBuffer>,
}

pub struct LUMem{
    pub lower: u32,
    pub upper: u32,
}

pub struct BootDevice{
    pub parts: [u8;4]
}

pub struct Modules{
    pub count: u32,
    pub addr:  u32,
    // TODO: заглушка
}

pub struct SymbolTable{
    pub num: [u32;4]
}

pub struct Frame{
    pub size:   u32,
    pub addr:   u32,
    _addr_high: u32,
    pub length: u32,
    _len_high:  u32,
    pub flag:   u32,
}

pub struct Drives{
    pub length: u32,
    pub addr:   u32, //Указатель на массив Drive
}
pub struct Drive<T>{
    pub number:    u8,
    pub mode:      u8,
    pub cylinders: u16,
    pub heads:     u8,
    pub sectors:   u8,
    pub ports:     T,
}

/*
pub struct BIOSConfigTable{
//TODO
}

pub struct APMTable{
    pub version:         u16,
    pub code_seg:        u16,
    pub offset:          u32,
    pub code_seg_16:     u16,
    pub data_seg:        u16,
    pub flags:           u16,
    pub code_seg_len:    u16,
    pub code_seg_16_len: u16,
    pub data_seg_len:    u16,
}

pub struct VBE{ 
    pub ctrl_info:     u32, // Формально, это 2 указателя на какие-то структуры. Пока не будем трогать
    pub mode_info:     u32,
    pub mode:          u16,
    pub interface_seg: u16,
    pub interface_off: u16,
    pub interface_len: u16,
}
*/
pub struct FrameBuffer{
    pub addr: u32,
    pub width:u32,
    pub height:u32,
    pub bpp:u8,
    pub flag:u8
    // TODO: should use "pitch" field?
}


impl MultiBoot{
    pub const fn new()->MultiBoot{
        MultiBoot{
            flags: 0,
            mem: None,
            boot: None,
            cmdline: None,
            mods: None,
            syms: None,
            mmap: None,
            drives: None,
            loader: None,
            fb: None
        }
    }
    pub fn init(&mut self,loader_info: &LoaderInfo){
        unsafe{
            self.flags=loader_info.flags();
            if self.flag(0){
                let temp=&loader_info.mem;
                self.mem=Some(LUMem{
                    lower: temp.lower,
                    upper: temp.upper
                });
            }
            if self.flag(1){
                self.boot=Some(BootDevice{
                    parts: loader_info.boot.parts
                });
            }
            if self.flag(2){
                let temp=to_str(loader_info.cmdline);
                self.cmdline=Some(temp);
                // TODO: deallocate
            }
            if self.flag(3){
                let temp=Modules{
                    count: loader_info.mods.count,
                    addr: loader_info.mods.addr
                };
                // TODO: deallocate
                self.mods=Some(temp);
            }
            if self.flag(4) || self.flag(5){
                self.syms=Some(SymbolTable{
                    num: loader_info.syms.num
                });
                // TODO: deallocate
            }
            if self.flag(6){
                let temp=&loader_info.mmap;
                self.mmap=Some(core::slice::from_raw_parts(temp.addr,temp.length/core::mem::size_of::<Frame>()));
                // TODO: deallocate
            }
            if self.flag(7){
                // TODO: how present info about drives?
            }
            if self.flag(8){
                // TODO
            }
            if self.flag(9){
                let temp=to_str(loader_info.loader);
                self.loader=Some(temp);
                // TODO: deallocate
            }
            if self.flag(10){
                // TODO
            }
            if self.flag(11){
                // TODO
            }
            if self.flag(12){
                self.fb=Some(FrameBuffer{
                    addr: loader_info.fb.addr,
                    width:loader_info.fb.width,
                    height:loader_info.fb.height,
                    bpp:loader_info.fb.bpp,
                    flag:loader_info.fb.flag
                });
            }
        }
    }
    pub fn flag(&self,flag: u8)->bool{
        self.flags &(1<<flag) !=0
    }
    pub fn get_fb(&self)->&FrameBuffer{
        if let Some(ref fb)=self.fb{
            fb
        }else{
            panic!();
        }
    }
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
