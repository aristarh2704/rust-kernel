use core::ptr::NonNull;
// Real memory descriptor
pub struct RMDescriptor{}

struct Frame{
    size: usize,
    next: Option<FrameOwner>
}
impl Frame{
    pub fn borrow_next(&mut self)->Option<&mut Frame>{
        match self.next{
            Some(ref mut owner)=>Some(owner.frame),
            None=>None
        }
    }
    // May panics, if self contains pointer to next
    pub fn set_next(&mut self,next:FrameOwner){
        match self.next{
            Some(_)=>panic!("Trying set next on Frame"),
            None=>{},
        };
        self.next=Some(next);
    }
    pub fn get_next(&mut self)->Option<FrameOwner>{
        self.next.take()
    }
}
impl Drop for Frame{
    fn drop(&mut self){
        panic!("Frame cannot be automaticaly dropped");
    }
}
struct FrameOwner{
    frame: &'static mut Frame
}
impl<'a> FrameOwner{
    pub fn to_descriptor(self)->(VMDescriptor,Option<FrameOwner>){
        let next=self.frame.next.take();
        let addr=self.frame as *mut Frame as usize;
        let size=self.frame.size;
        core::mem::forget(self);
        (VMDescriptor{size:size,addr:addr},next)
    }
    pub fn try_from_descriptor(data:VMDescriptor)->Result<FrameOwner,VMDescriptor>{
        if data.size>=8{
            let frame_ptr=unsafe{&mut *(data.addr as *mut Frame)};
            frame_ptr.size=data.size;
            frame_ptr.next=None;
            core::mem::forget(data);
            Ok(FrameOwner{frame:frame_ptr})
        }else{
            Err(data)
        }
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
            core::mem::forget(self);
            return result;
        }else{
            Err(self)
        }
    }
    pub fn combine(self,next:Self)->Result<Self,(Self,Self)>{
        if self.size+self.addr==next.addr{
            Ok(VMDescriptor{size:self.size+next.size,addr:self.addr})
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
}
impl Drop for VMDescriptor{
    fn drop(&mut self){
        panic!("VMDescriptor cannot be automaticaly dropped");
    }
}

struct Owned<T>{
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
#[alloc_error_handler]
fn oom(_layout: alloc::alloc::Layout) -> ! {
    panic!("seems oom occured");
}
