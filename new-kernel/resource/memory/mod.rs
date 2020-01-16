use core::ptr::NonNull;
use core::mem::forget;
// Real memory descriptor
pub struct RMDescriptor{}

struct Frame{
    size: usize,
    next: Option<FrameOwner>
}
impl Drop for Frame{
    fn drop(&mut self){
        panic!("Frame cannot be automaticaly dropped");
    }
}

// We can ensure that pointed Frame have not next
struct CreatedFrame{
    frame: &'static mut Frame
}
impl CreatedFrame{
    pub fn to_frameowner(self)->FrameOwner{
        let frame=unsafe{&mut *(self.frame as *mut Frame)};
        forget(self);
        FrameOwner{frame:frame}
    }
    pub fn set_next(self,next:FrameOwner)->FrameOwner{
        self.frame.next=Some(next);
        let frame=unsafe{&mut *(self.frame as *mut Frame)};
        forget(self);
        FrameOwner{frame:frame}
    }
}
impl Drop for CreatedFrame{
    fn drop(&mut self){
        // TODO
    }
}

pub struct FrameOwner{
    frame: &'static mut Frame
}
impl<'a> FrameOwner{
    pub fn to_descriptor(self)->(VMDescriptor,Option<FrameOwner>){
        let next=self.frame.next.take();
        let addr=self.frame as *mut Frame as usize;
        let size=self.frame.size;
        forget(self);
        (VMDescriptor{size:size,addr:addr},next)
    }
    pub fn get_next(&mut self)->Option<FrameOwner>{
        self.frame.next.take()
    }
    pub fn borrow_next(&mut self)->&mut Option<FrameOwner>{
        &mut self.frame.next
    }
    pub fn size(&self)->usize{
        self.frame.size
    }
    pub fn addr(&self)->usize{
        &*self.frame as *const Frame as usize
    }
}
impl Drop for FrameOwner{
    fn drop(&mut self){
        panic!("FrameOwner cannot be automaticaly dropped");
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
    pub fn convert<T>(self)->Result<Owned<T>,Self>{ // TODO: memory uninitialized
        if core::mem::size_of::<T>()==self.size{
            Ok(Owned{data:unsafe{NonNull::new_unchecked(self.addr as *mut T)}})
        }else{
            Err(self)
        }
    }
    pub fn try_to_frameowner(self){
        
    }
}
impl Drop for VMDescriptor{
    fn drop(&mut self){
        panic!("VMDescriptor cannot be automaticaly dropped");
    }
}

pub struct Owned<T>{
    data: NonNull<T>
}
struct Page{
    // attributes
}
pub fn pages_from_bootinfo()->(){} // Return page iterator.
// pub fn mmap(Page,address,attrs,page_allocator){};
//#![feature(alloc)]
extern crate alloc;
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
