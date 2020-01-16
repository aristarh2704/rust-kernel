use core::ptr::NonNull;
use core::mem::forget;
use alloc::boxed::Box;
// Real memory descriptor
pub struct RMDescriptor{}

struct Frame{
    size: usize,
    next: Option<Box<Frame>> // Box is only container of Frame, it's content not allocated
}
impl Frame{
    pub fn to_descriptor(mut frame:Box<Frame>)->(VMDescriptor,Option<Box<Frame>>){
        let size=frame.size;
        let next=frame.next.take();
        let addr=Box::<Frame>::into_raw(frame) as usize;
        (VMDescriptor{size:size,addr:addr},next)
    }
    pub fn addr(&self)->usize{
        self as *const Frame as usize
    }
    pub fn size(&self)->usize{
        self.size
    }
    // Next frame must not contain Box
    pub fn merge(&mut self,next:Box<Frame>)->Result<(),Box<Frame>>{
        match next.next{
            Some(_)=>{return Err(next);},
            None=>{},
        };
        if (self as *mut Frame as usize)+self.size!=next.addr(){
            return Err(next);
        }
        self.size+=next.size;
        forget(next);
        Ok(())
    }
    pub fn set_next(&mut self,next:Box<Frame>)->Result<(),Box<Frame>>{
        match self.next{
            Some(_)=>{return Err(next);},
            None=>{},
        };
        self.next=Some(next);
        Ok(())
    }
    // Before: A->B, C-> must be none
    // After: A->C->B
    pub fn insert(&mut self, mut next:Box<Frame>)->Result<(),Box<Frame>>{
        match next.next{
            Some(_)=>{return Err(next);},
            None=>{},
        };
        let next2=self.next.take();
        next.next=next2;
        self.next=Some(next);
        Ok(())
    }
    pub fn next(&mut self)->&mut Option<Box<Frame>>{
        &mut self.next
    }
    pub fn get_next(&mut self)->Option<Box<Frame>>{
        self.next.take()
    }
}
impl Drop for Frame{
    fn drop(&mut self){
        panic!("Frame cannot be automaticaly dropped");
    }
}

// Virtual memory descriptor
pub struct VMDescriptor{
    size: usize,
    addr: usize
}
impl VMDescriptor{
    pub fn cut(self,first:usize)->Result<(Self,Self),Self>{
        if self.size>first && first!=0{
            let result=Ok((
                VMDescriptor{size:first,addr:self.addr},
                VMDescriptor{size:self.size-first,addr:self.addr+first}));
            forget(self);
            result
        }else{
            Err(self)
        }
    }
    pub fn combine(self,next:Self)->Result<Self,(Self,Self)>{
        if self.size+self.addr==next.addr{
            let result=Ok(VMDescriptor{size:self.size+next.size,addr:self.addr});
            forget(self);
            forget(next);
            result
        }else{
            Err((self,next))
        }
    }
    pub fn try_to_frame(self)->Result<Box<Frame>,Self>{
        if self.size>=core::mem::size_of::<Frame>(){
            let frame=unsafe{&mut *(self.addr as *mut Frame)};
            frame.size=self.size;
            frame.next=None;
            let frame=self.addr as *mut Frame;
            forget(self);
            forget(frame);
            Ok(unsafe{Box::from_raw(frame)})
        }else{
            Err(self)
        }
    }
}
impl Drop for VMDescriptor{
    fn drop(&mut self){
        panic!("VMDescriptor cannot be automaticaly dropped");
    }
}

struct Page{
    // attributes
}
pub fn pages_from_bootinfo()->(){} // Return page iterator.
// pub fn mmap(Page,address,attrs,page_allocator){};
//#![feature(alloc)]
extern crate linked_list_allocator;
mod init;
pub use self::init::{init, MemoryRegion};
/*pub struct PhisRegion{
    size: usize,
    addr: usize,
    accessMode: AccessMode
}
pub fn allocate_phys_reg(address: usize /* maybe 0, if device mem maybe at any position */, count: usize /* count of pages */)->PhisPages{loop{}}
pub fn allocate_mem_reg(count: usize)->MemPages;
trait AddressSpace{
    fn state(usize)->PageState;
}
enum State{
    NotPresent, //Region is in address space, but not presented in phys memory
    NotAllowed, // Special descriptors, not memory
    Present, // Region is in phys memory
    Empty // Region not presented in address space
}*/
#[global_allocator]
pub static ALLOCATOR: linked_list_allocator::LockedHeap =
    linked_list_allocator::LockedHeap::empty();
/*
pub struct AllocatorProxy{}
impl core::alloc::GlobalAlloc for AllocatorProxy{
}
#[global_allocator]
pub static ALLOCATOR: AllocatorProxy=AllocatorProxy{};
*/
#[alloc_error_handler]
fn oom(_layout: alloc::alloc::Layout) -> ! {
    panic!("seems oom occured");
}
