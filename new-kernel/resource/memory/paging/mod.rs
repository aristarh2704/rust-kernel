/*
 * This module works with descriptor tables
 * Requires from memory module: TODO
 * Kernel allocator may allocate pages in kernel VMA
 * Process can create mmap from file or shared map, require new pages for code/data
 * Sheduler can switch to another address space
 * Each proccess has own virtual address space
 * Simple model: heap from begin, stack from end
 * All mmaps are created from heap end, so brk/sbrk cannot be implemented
 */
#[macro_use]
extern crate bitflags;

// All changes passed to arch-specific hadler, it may reload tlb entries in proccessor
/*
fn change(&mut self,new_type)->Result;
    fn mmap(&mut self,VMADescriptor /* it can be from any space, include self */,mode)->Result;
    fn write(&mut self,Buffer)->Result;
    fn read(&mut self,Buffer)->Result;
*/

struct VMASpace{
    table: arch::resource::memory::paging::VMASpace,
    pages: Box<[Page]>,
    swapper: ???
}
impl VMASpace{
    fn new()->Self{
    }
    fn pages(&mut self)->&mut [Page]{
        self.pages.borrow_mut()
    }
}
struct Page{
    location: Location,
    mode: Mode,
}
bitflags!{
    struct Mode: u8{
        const KERNEL=0;
        const USER=1;
        const READ=2;
        const WRITE=4;
        const EXEC=8;
    }
}
enum Location{
    NotUsed,
    Empty,
    PhysMapped(PhysLocker),
    Swapped(SwapIdentifier)
}
impl PhysPageUser for Location{
    fn free(&mut self)->PhysLocker{
    }
}
