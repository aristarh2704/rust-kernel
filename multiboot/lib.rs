#![no_std]
#![feature(const_fn)]
extern crate spin;
#[macro_use]
extern crate devices;
// Description of Multiboot information structure is in 
// https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Boot-information-format
//mod boot;

pub struct MultiBoot{
    pub cmdline:Option<&'static str>,
    pub mmap:   Option<&'static [Frame]>,
    pub loader: Option<&'static str>,
    pub fb:     FrameBuffer,
}
#[repr(packed)]
pub struct Frame{
    pub addr:   u32,
    _addr:      u32,
    pub length: u32,
    _length: u32,
    pub flag:   u32,
    _reserved:  u32
}

pub struct FrameBuffer{
    pub addr: u32,
    pub width:u32,
    pub height:u32,
    pub bpp:u8,
    pub flag:u8
    // TODO: should use "pitch" field?
}

struct Addr{
    x: usize,
    readed:usize
}
impl Addr{
    fn new(x:usize)->Addr{
        Addr{
            x:x,
            readed:0
        }
    }
    fn read<'a,'b,T>(&'a mut self)->&'b mut T{
        let addr=unsafe{
            &mut *(self.x as *mut T)
        };
        self.x+=core::mem::size_of::<T>();
        self.readed+=core::mem::size_of::<T>();
        addr
    }
    fn add(&mut self,x:usize){
        self.x+=x;
        self.readed+=x;
    }
}

impl MultiBoot{
    pub const fn new()->MultiBoot{
        MultiBoot{
            cmdline: None,
            mmap: None,
            loader: None,
            fb: FrameBuffer{
                addr: 0xb8000,
                height: 25,
                width: 80,
                bpp: 2,
                flag:2
            }
        }
    }
    pub fn init(&mut self,loader_info: usize){
        let mut base=Addr::new(loader_info);
        let size=*base.read::<usize>();
        base.read::<u32>();
        debug!("MBI tags: ");
        while base.readed<size{
            let last=base.readed;
            let flag=*base.read::<u32>();
            let sub_size=*base.read::<u32>();
            debug!("{} ",flag);
            match flag{
                8=>{
                    let addr=*base.read::<u32>();
                    base.read::<u32>();
                    base.read::<u32>();
                    let width=*base.read::<u32>();
                    let height=*base.read::<u32>();
                    let bpp=*base.read::<u8>();
                    let flag=*base.read::<u8>();
                    self.fb=FrameBuffer{
                        addr:   addr,
                        width:  width,
                        height: height,
                        bpp:    bpp,
                        flag:   flag
                    };
                }
                6=>{
                    let addr=base.x+8;
                    let count=((sub_size-16)/24) as usize;
                    self.mmap=Some(unsafe{core::slice::from_raw_parts_mut(addr as *mut Frame,count)});
                }
                _=>{}
            }
            let aligned=((sub_size-1)/8)*8+8;
            let must_read=(aligned as usize)-(base.readed-last);
            base.add(must_read);
        }
        debug!("\n");
    }
}
pub fn init(loader_info:usize)->MultiBoot{
    let mut mb_info=MultiBoot::new();
    mb_info.init(loader_info);
    mb_info
}
