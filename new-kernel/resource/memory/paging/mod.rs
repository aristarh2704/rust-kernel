/*
 * This module works with descriptor tables
 * Requires from memory module: TODO
 * 
 */
trait Pager{
    // count is a num of needed pages
    fn get_memory(&mut self,size:usize,thread_table: ThreadPageData,type:MemType,mode: AccessMode)->Result<ThreadAssignedMemory,()>;
    fn free_memory(&mut self,MemoryDescriptor,thread_identifier);
    fn mmap(&mut self,MemoryDescriptor,owner_thread_table:ThreadPageData,target_thread_table:ThreadPageData,type: MemType,mode:AccessMode);
}
enum MemType{
    Code,
    Data,
    Stack
}
enum AccessMode{
    Read,
    Write,
    Exec
}
// This struct represents pages, allocated for thread or process
// Pages may be returned through api module to user process as raw memory block.
struct ThreadAssignedMemory{
    addr: usize, // must be aligned to page size
    count: usize
    //owner_thread: ???
}

struct ThreadPageData{
    data: arch::resource::memory::paging::ThreadPageData
}