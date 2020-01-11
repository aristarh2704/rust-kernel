// Real memory descriptor
pub struct RMDescriptor{}

// Virtual memory descriptor
pub struct VMDescriptor{
    size: usize,
    addr: usize
}
impl VMDescriptor{
    pub fn cut(self,first:usize)->Result<(Self,Self),Self>{
        if self.size>first && first!=0{
            Ok((
                VMDescriptor{size:first,addr:self.addr},
                VMDescriptor{size:self.size-first,addr:self.addr+first}))
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
            Ok(Owned{data:self.addr as *mut T})
        }else{
            Err(self)
        }
    }
}
struct Owned<T>{
    data: *mut T
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
