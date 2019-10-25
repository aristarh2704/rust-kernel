#![feature(alloc,alloc_error_handler)]
#![feature(alloc_error_handler)]
extern crate linked_list_allocator;
extern crate alloc;
pub mod init;
pub use self::init::init;
#[global_allocator]
pub static ALLOCATOR: linked_list_allocator::LockedHeap=linked_list_allocator::LockedHeap::empty();
#[alloc_error_handler]
fn oom(_layout: alloc::alloc::Layout) -> ! {
    panic!("seems oom occured");
}
