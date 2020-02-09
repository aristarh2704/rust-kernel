use super::linked_list_allocator::LockedHeap;
use core::alloc::GlobalAlloc;
use resource::memory::ALLOCATOR;
pub struct BootRegion {
    pub base: usize,
    pub size: usize,
    pub type: BootRegionType
}
pub unsafe fn init<T: Iterator<Item = BootRegion>>(regions: T) {
    for reg in regions {
        ALLOCATOR.dealloc(
            reg.base as *mut u8,
            core::alloc::Layout::from_size_align(reg.size, 1).unwrap(),
        );
    }
}
