pub type MultibootPointer = usize;
pub struct MultiBoot {
    pub mmap: Option<&'static [Frame]>,
    pub fb: FrameBuffer,
    pub stable: Option<SecTable>,
}
#[repr(packed)]
pub struct SecTable {
    pub index: u32, // ?
    pub entries: &'static [SecEntry],
}
#[repr(packed)]
#[derive(Debug)]
pub struct SecEntry {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u32,
    sh_addr: u32,
    sh_offset: u32,
    sh_size: u32,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u32,
    sh_entsize: u32,
}
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Frame {
    pub addr: u32,
    _addr: u32,
    pub length: u32,
    _length: u32,
    pub flag: u32,
    _reserved: u32,
}
pub fn parse(x: MultibootPointer) -> MultiBoot {
    unsafe {
        let mut base = Addr::new(x);
        let size = *base.read::<usize>();
        debug!(
            "Multiboot info: 0x{:08X}-0x{:08X}\n",
            base.x - 4,
            base.x - 4 + size
        );
        base.read::<u32>();
        let mut mb = MultiBoot {
            mmap: None,
            fb: FrameBuffer {
                addr: 0xb8000,
                height: 25,
                width: 80,
                bpp: 2,
                flag: 2,
            },
            stable: None,
        };
        while base.readed < size {
            let last = base.readed;
            let flag = *base.read::<u32>();
            let sub_size = *base.read::<u32>();
            match flag {
                8 => {
                    let addr = *base.read::<u32>();
                    base.read::<u32>();
                    base.read::<u32>();
                    let width = *base.read::<u32>();
                    let height = *base.read::<u32>();
                    let bpp = *base.read::<u8>();
                    let flag = *base.read::<u8>();
                    mb.fb = FrameBuffer {
                        addr: addr,
                        width: width,
                        height: height,
                        bpp: bpp,
                        flag: flag,
                    };
                }
                6 => {
                    let addr = base.x + 8;
                    let count = ((sub_size - 16) / 24) as usize;
                    mb.mmap =
                        Some(unsafe { core::slice::from_raw_parts_mut(addr as *mut Frame, count) });
                }
                9 => {
                    // In multiboot documentation fields are 16 bit size,
                    // but in real it's 32 bit
                    let num = *base.read::<u32>();
                    let entsize = *base.read::<u32>();
                    let shndx = *base.read::<u32>();
                    //base.read::<u32>();
                    debug!("Entries start: 0x{:08X}\n", base.x);
                    mb.stable = Some(SecTable {
                        index: shndx,
                        entries: unsafe {
                            core::slice::from_raw_parts(base.x as *const SecEntry, num as usize)
                        },
                    });
                    if let Some(ref table) = mb.stable {
                        for i in 0..num as usize {
                            debug!("{:?}\n", table.entries[i]);
                        }
                    };
                }
                _ => {}
            }
            let aligned = ((sub_size - 1) / 8) * 8 + 8;
            let must_read = (aligned as usize) - (base.readed - last);
            base.add(must_read);
        }
        mb
    }
}

pub struct FrameBuffer {
    pub addr: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub flag: u8, // TODO: should use "pitch" field?
}
struct Addr {
    x: usize,
    readed: usize,
}
impl Addr {
    fn new(x: usize) -> Addr {
        Addr { x: x, readed: 0 }
    }
    fn read<'a, 'b, T>(&'a mut self) -> &'b mut T {
        let addr = unsafe { &mut *(self.x as *mut T) };
        self.x += core::mem::size_of::<T>();
        self.readed += core::mem::size_of::<T>();
        addr
    }
    fn add(&mut self, x: usize) {
        self.x += x;
        self.readed += x;
    }
}
use crate::resource::memory::MemoryRegion;
pub struct RegionIterator<'a> {
    index: usize,
    mmap: &'a Option<&'static [Frame]>,
}
#[no_mangle]
extern{
    pub static kernel_end:usize;
}
impl<'a> Iterator for RegionIterator<'a> {
    type Item = MemoryRegion;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.index + 1;
            if self.mmap.unwrap().len() <= index {
                return None;
            }
            let mut fr = self.mmap.unwrap()[index];
            self.index += 1;
            if fr.flag == 1 {
                let kern_end:usize = unsafe{&kernel_end as *const usize as usize};
                if fr.addr==0x100000{
                    fr.addr==kern_end as u32;
                    fr.length==fr.length-kern_end as u32 +0x100000;
                }
                debug!(
                    "Add region: 0x{:08X}-0x{:08X}\n",
                    fr.addr,
                    fr.addr + fr.length
                );
                return Some(MemoryRegion {
                    base: fr.addr as usize,
                    size: fr.length as usize,
                });
                
            }
        }
    }
}
impl MultiBoot {
    pub fn regions<'a>(&self) -> RegionIterator {
        RegionIterator {
            index: 0,
            mmap: &self.mmap,
        }
    }
}
