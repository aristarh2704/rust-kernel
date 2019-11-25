#![feature(alloc)]
extern crate alloc;
extern crate linked_list_allocator;
mod init;
pub use self::init::{init, MemoryRegion};
#[global_allocator]
pub static ALLOCATOR: linked_list_allocator::LockedHeap =
    linked_list_allocator::LockedHeap::empty();
#[alloc_error_handler]
fn oom(_layout: alloc::alloc::Layout) -> ! {
    panic!("seems oom occured");
}
