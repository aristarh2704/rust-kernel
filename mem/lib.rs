#![no_std]
extern crate spin;
use spin::Mutex;
pub struct Heap{
    area: Option<&'static mut Area>,
}
#[repr(packed)]
struct Area{
    size: u32,
    next: Option<&'static mut Area>
}
/*
impl Heap{
    fn alloc<'a,'b,T>(&'a mut self, size: u32,align: u8)->&'b mut T{
        if align>20{
            panic!()
        }
        let mut parea=&mut self.area;
        let align:u32=1<<align;
        loop{
            let area=match parea{
                &mut Some(ref n)=>n,
                &mut None=>panic!()
            };
            let addr=(*area as *mut Area) as u32;
            let end=area.size+addr;
            let rstart=addr&align;
            if(end>(rstart+size)){
                *parea=area.next.clone();
                if (rstart-addr)>=core::mem::size_of::<Area>() as u32{
                }
            }
        }
    }
}
*/