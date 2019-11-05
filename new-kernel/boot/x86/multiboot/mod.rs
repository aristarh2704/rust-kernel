pub type MultibootPointer=usize;
pub struct MultiBoot {
    pub mmap: Option<&'static [Frame]>,
    pub fb: FrameBuffer,
    pub stable: Option<SecTable>
}
#[repr(packed)]
pub struct SecTable{
    pub index: u16, // ?
    pub entries: &'static [SecEntry]
}
#[repr(packed)]
pub struct SecEntry{
  sh_name: u32,
  sh_type:u32,
  sh_flags:u32,
  sh_addr:u32,
  sh_offset:u32,
  sh_size:u32,
  sh_link:u32,
  sh_info:u32,
  sh_addralign:u32,
  sh_entsize:u32
} 
#[repr(packed)]
#[derive(Copy,Clone)]
pub struct Frame {
    pub addr: u32,
    _addr: u32,
    pub length: u32,
    _length: u32,
    pub flag: u32,
    _reserved: u32,
}
pub fn parse(x: MultibootPointer)->MultiBoot{
	unsafe{
		let mut base = Addr::new(x);
        let size = *base.read::<usize>();
        debug!("Multiboot info: 0x{:08X}-0x{:08X}\n",base.x-4,base.x-4+size);
        base.read::<u32>();
		let mut mb=MultiBoot{
			mmap: None,
			fb:FrameBuffer{
				addr: 0xb8000,
                height: 25,
                width: 80,
                bpp: 2,
                flag: 2,
			},
            stable:None
		};
        while base.readed < size {
            let last = base.readed;
            let flag = *base.read::<u32>();
            let sub_size = *base.read::<u32>();
            debug!("Flag: {}\n",flag);
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
                    debug!("We here\n");
                    //let num=*base.read::<u16>();
                    //let ensize=*base.read::<u16>();
                    //let shndx=*base.read::<u16>();
                    debug!("We here\n");
                    //debug!("Count: {}, size: {}, shndx: {}, struct_size: {}\n",num,ensize,shndx,core::mem::size_of::<SecEntry>());
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
    pub addr: u32,                                 pub width: u32,
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

pub struct RegionIterator<'a>{
	index: usize,
	mmap: &'a Option<&'static [Frame]>
}
		impl<'a> Iterator for RegionIterator<'a>{
			type Item=crate::resource::paging::MemoryRegion;
			fn next(&mut self)->Option<Self::Item>{
                loop{
				let index=self.index+1;
				if self.mmap.unwrap().len()<=index{return None;}
				let fr=self.mmap.unwrap()[index];
                self.index+=1;
                if(fr.flag==1){
                    debug!("Add region: 0x{:08X}-0x{:08X}\n",fr.addr,fr.addr+fr.length);
	    			return Some(crate::resource::paging::MemoryRegion{
					base: fr.addr as usize,
					size: fr.length as usize
		    		});
                }
                }
			}
		}
impl MultiBoot{
	pub fn regions<'a>(& self)->RegionIterator{
		RegionIterator{
			index:0,
			mmap: &self.mmap,
		}
	}
}
