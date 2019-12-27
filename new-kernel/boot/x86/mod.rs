#[macro_use]
pub mod early_console;
mod multiboot;
#[no_mangle]
pub unsafe extern "C" fn entry(x: multiboot::MultibootPointer) {
    let boot_info = multiboot::parse(x);
    crate::resource::memory::init(boot_info.regions());
    early_console::init();
    debug!("Kernel booted\n");
    /*debug!("Symtab: 0x{:08X}, end: 0x{:08X}\n",&strtab as *const _ as usize,&strtab_e as *const _ as usize);
    debug!("Kernel end: 0x{:08X}\n",code_e as usize);
    let mystr:*const u8=&strtab as *const _ as *const u8;
    for i in 0..100{
        debug!("{}",core::ptr::read::<u8>(mystr.offset(i)) as char);
    }*/
}
