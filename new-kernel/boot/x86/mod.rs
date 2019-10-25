mod multiboot;
#[macro_use]
pub mod early_console;
#[no_mangle]
pub unsafe extern "C" fn entry(x: multiboot::MultibootPointer,cs:usize,ce:usize,ss:usize,se:usize,ds:usize,de:usize){
	let boot_info=multiboot::parse(x);
	crate::resource::memory::init(boot_info.regions(cs,ce,ss,se,ds,de));
	early_console::init();
	debug!("Kernel booted");
}
