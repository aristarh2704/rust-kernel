#![no_std]
#[no_mangle]
extern "C" {
    //pub fn in_fn(port: u16)->u8;
    pub fn out_fn(port: u16, byte: u8);
}
