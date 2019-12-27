/*
 * This module works with memory, it's obvious.
 * Components: allocator, paging.
 * API: address space alloc (raw pointer), phys alloc (PhysReg), set the special descriptor, work
 * with mmaps, set page attributes (cow, etc), callbacks (swapper).
 * Allocator return Frame struct, which may be converted to any type.
 * Address space: array of accessible regions (virtual addresses).
 * If allocator can't alloc memory, it tries increase address space.
 * Newly created region after this must be mapped to phys mem to be accessible.
 * UsedMem: consist all ram pages descriptors (physical addresses), number of uses, and pointers to address space descriptors.
 * PhysMem: consist list of free phys regions. Needed for devices.
 */

 // Contains data, which cannot be swapped and moved.
struct PhysContainer{} 

// Each address space must contain one descriptor for all reachable memory
struct AddressSpace{
    as: paging::AddressSpace,
    free: LinkedList<VirtualRegion>
}
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
