struct Frame<T>{
    size: usize, // size of this peace of memory
    content: T // can contain pointer to next Frame. It's type defined by allocator.
}
// TODO: Frame can be cutted to two parts or united from two contiguous Frames.
// Frame can be converted to any type, before it must be zeroed.

struct Page{
    // attributes
}
pub fn pages_from_bootinfo()->(){} // Return page iterator.
pub fn mmap(Page,address,attrs,page_allocator);
#![feature(alloc)]
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
