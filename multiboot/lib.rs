#![no_std]
// Description of Multiboot information structure is in 
// https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Boot-information-format

#[derive(Default)]
pub struct MultiBoot<'a>{
    flags:u32,
    //mem:    Option<LUMem>,
    //boot:   Option<BootDevice>,
    //cmdline:Option<&'a str>,
    //mods:   Option<&'a[Modules]>,
    //syms:   Option<&'a SymbolTable>,   // Я не знаю, что с этим делать
    //mmap:   Option<&'a[Mmap]>,
    //drives: Option<'a[Drive]>,
    //config: Option<&'a ConfigTable>,
    loader: Option<&'a str>,
    //apm:    Option<&'a ApmTable>,
    //vbe:    Option<VBE>,
    //fb:     Option<FrameBuffer>,
}
pub static MULTIBOOT:MultiBoot=MultiBoot{
    flags:0,
    loader:None
};
