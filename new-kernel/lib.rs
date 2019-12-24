#![no_std]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
mod arch;
mod resource;
#[macro_use]
mod boot;
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {
    debug!("eh_personality");
    loop {}
}
#[panic_handler]
#[no_mangle]
pub unsafe fn panicimpl(x: &core::panic::PanicInfo) -> ! {
    let mut ebp = (&x as *const _ as usize) - 8;
    for i in 0..20 {
        let eip = *((ebp + 4) as *const usize);
        if eip == 0 {
            break;
        }
        ebp = *(ebp as *const usize);
        debug!("0x{:08X}", eip);
        debug!("\n");
    }
    debug!("Kernel panic: {}\n", x);
    loop {}
}
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {
    debug!("Unwind_Resume");
    loop {}
}
