#[macro_use]
pub mod early_console;
mod multiboot;
#[no_mangle]
pub unsafe extern "C" fn entry(x: multiboot::MultibootPointer,cs:usize,ce:usize,ss:usize,se:usize,ds:usize,de:usize){
	let boot_info=multiboot::parse(x);
    debug!("Code:   0x{:08X}-0x{:08X}\n",cs,ce);
    debug!("Data:   0x{:08X}-0x{:08X}\n",ds,de);
    debug!("Stack:   0x{:08X}-0x{:08X}\n",ss,se);
	crate::resource::memory::init(boot_info.regions(cs,ce,ss,se,ds,de));
	early_console::init();
	debug!("Kernel booted");
}
