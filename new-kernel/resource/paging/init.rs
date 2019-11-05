pub struct MemoryRegion{
    pub base: usize,
    pub size: usize
}
pub unsafe fn init<T:Iterator<Item=MemoryRegion>>(regions:T){
    for reg in regions{
            //do_nothing;
    }
}
