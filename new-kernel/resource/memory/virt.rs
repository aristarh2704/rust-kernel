use core::ptr::NonNull;
use core::mem::forget;
use alloc::boxed::Box;
pub mod lock;
pub struct Frame{
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
    // Set next field, return previous value
    pub fn swap_next(&mut self,next:Box<Frame>)->Option<Box<Frame>>{
        self.next.replace(next)
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
        panic!("Frame are leaked");
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
        panic!("VMDescriptor are leaked");
    }
}

trait KernelAllocator{
    fn alloc(&mut self,size: usize,align:usize)->Result<VMDescriptor,()>; // If VMDescriptor is not suitable, it returns to allocator. I think this situation will not happen
    fn dealloc(&mut self,data: VMDescriptor);
    fn min_align(&self)->usize; // Called at once, at setting allocator
    fn min_size(&self)->usize; // Above
    fn on_drop_frame(&mut self,Box<Frame>); // If Frame dropped, it's returns to allocator, because dropping is a bad situation
    fn sort_internal(&mut self){return;} // Used for sorting internal structures, if allocator does not doing it automatically
}
pub struct AllocatorProxy{
    min_align: usize,
    min_size: usize,
    allocator: lock::spin::Mutex<Option<Box<KernelAllocator + Send + Sync>>>
}
impl AllocatorProxy{
    pub fn try_set(&self,allocator:Box<KernelAllocator + Send + Sync>)->Result<(),Box<KernelAllocator + Send + Sync>>{
        let mut_self=unsafe{&mut *(self as *const Self as *mut Self)};
        let mut locker=mut_self.allocator.lock();
        match locker.deref_mut(){
            Some(_)=>return Err(allocator),
            None=>(),
        };
        mut_self.min_align=allocator.min_align();
        mut_self.min_size=allocator.min_size();
        locker.deref_mut().replace(allocator);
        return Ok(());
    }
    // TODO: change_allocator at runtime
}
use alloc::alloc::{GlobalAlloc,Layout};
use core::ops::DerefMut;
unsafe impl GlobalAlloc for AllocatorProxy{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8{
        let mut_self=&mut *(self as *const Self as *mut Self);
        match mut_self.allocator.lock().deref_mut(){
            Some(ref mut n)=>{ // n is &mut Box<>
                let mut size=layout.size();
                let mut align=layout.align();
                size=if size<self.min_size{self.min_size}else{size};
                align=if align<self.min_align{self.min_align}else{align};
                let desc=n.alloc(size,align).unwrap_or(VMDescriptor{addr:0,size:0});
                if desc.size==0{
                    return 0 as *mut u8;
                };
                if desc.size!=size{
                    panic!("Allocator returns broken descriptor");
                }
                let result=desc.addr as *mut u8;
                forget(desc);
                return result;
                
            },
            None=>panic!("Allocator not setted")
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout){
        let mut_self=&mut *(self as *const Self as *mut Self);
        match mut_self.allocator.lock().deref_mut(){
            Some(ref mut n)=>{ // n is &mut Box<>
                let mut size=layout.size();
                let mut align=layout.align();
                size=if size<self.min_size{self.min_size}else{size};
                align=if align<self.min_align{self.min_align}else{align};
                n.dealloc(VMDescriptor{addr:ptr as usize,size:size});                
            },
            None=>panic!("Allocator not setted")
        }
    }
}
//#[global_allocator]
static ALLOCATOR2: AllocatorProxy=AllocatorProxy{
    min_align:0,min_size:0,allocator:lock::spin::Mutex::new(None)
};

extern crate linked_list_allocator;
mod init;
pub use self::init::{init, MemoryRegion};
#[global_allocator]
static ALLOCATOR: linked_list_allocator::LockedHeap =
    linked_list_allocator::LockedHeap::empty(); // Will be deleted
#[alloc_error_handler]
fn oom(_layout: alloc::alloc::Layout) -> ! {
    panic!("seems oom occured");
}
