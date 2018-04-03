#![no_std]
extern crate spin;
use spin::Mutex;
pub struct Heap{
    area: *mut Area,
}
pub struct Owned<T:'static>{
    pointer: *mut T
}

struct Area{
    size: usize,
    next: *mut Area
}
// All raw pointers to unknown type are interpreted as *const u32 or &u32
const MIN_SIZE:usize=core::mem::size_of::<Area>();
const MIN_ALIGN:usize=core::mem::align_of::<Area>(); // I think, MIN_SIZE > MIN_ALIGN. But if MIN_SIZE<MIN_ALIGN?

impl Heap{
    fn dealloc(&mut self,pointer: *mut Area,mut size: usize){
        size=align_fn(size,MIN_ALIGN);
        unsafe{
            (*pointer).next=self.area;
            (*pointer).size=size;
        }
        self.area=pointer;
    }
    fn alloc(&mut self,mut size: usize,mut align: usize)->*mut u32{
        if align>20{
            return 0 as *mut u32
        }
        let mut area: *mut*mut Area = if (self.area as usize)==0 {
            return 0 as *mut u32
        }else{
            &mut self.area
        };
        align = 1<<align;
        align = core::cmp::max(align,MIN_ALIGN);
        size = core::cmp::max(size,MIN_SIZE);
        size = align_fn(size,align);
        loop{
            let start;
            let end;
            unsafe{
                start=*area as usize;
                end = (**area).size+start;
            }
            let rstart=align_fn(start,align);
            if rstart+size>end{
                unsafe{
                    area= if ((**area).next as usize)==0 {
                        return 0 as *mut u32
                    }else{
                        &mut (**area).next
                    };
                }
            }else{
                if rstart>start{
                    unsafe{(**area).size=rstart-start;}
                }else{
                    unsafe{
                        *area=(**area).next;
                    }
                };
                if rstart+size<end{
                    self.dealloc((rstart+size) as *mut Area,end-rstart-size);
                }
                return rstart as *mut u32;
            }            
        }
    }
    pub unsafe fn add_region(&mut self, mut addr: usize, mut size: usize){
        size=(size/MIN_SIZE)*MIN_SIZE;
        if size<MIN_SIZE{
            return;
        }
        addr=align_fn(addr,MIN_ALIGN);
        self.dealloc(addr as *mut Area,size);
    }
}
unsafe impl core::marker::Send for Heap{}
pub static HEAP: Mutex<Heap>=Mutex::new(Heap{area:0 as *mut Area});

fn align_fn(size: usize,align: usize)->usize{
    (size+align-1)& !(align-1)
}

impl<T> Drop for Owned<T>{
    fn drop(&mut self){
        unsafe{
            core::ptr::drop_in_place(self.pointer);
            HEAP.lock().dealloc(self.pointer as *mut Area,core::mem::size_of::<T>());
        }
    }
}