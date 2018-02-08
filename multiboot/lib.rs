#![no_std]
extern crate spin;
// Description of Multiboot information structure is in 
// https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Boot-information-format

#[repr(packed)]
pub struct MultiBoot<'a>{
    flags:  u32,
    mem:    LUMem,
    boot:   &'a BootDevice,
    cmdline: *const u8,
    mods:   Modules,
    syms:   SymbolTable,
    pub mmap:   Mmap,
    drives: Drives,
    config: &'a BIOSConfigTable,
    pub loader: *const u8,
    apm:    &'a APMTable,
    vbe:    VBE,
    //fb:     Option<FrameBuffer>,
}

#[repr(packed)]
pub struct LUMem{
    pub lower: u32,
    pub upper: u32,
}

#[repr(packed)]
pub struct BootDevice{
    pub part3: u8,
    pub part2: u8,
    pub part1: u8,
    pub drive: u8,
}

#[repr(packed)]
pub struct Modules{
    pub mods_count: u32,
    pub mods_addr:  u32,
    // TODO: заглушка
}

#[repr(packed)]
pub struct SymbolTable{
    pub num: [u32;4]
}

#[repr(packed)]
pub struct Mmap{
    pub length: u32,
    pub addr:   u32, //Указатель на массив Frame
}

pub struct Frame{
    pub size:   u32,
    pub addr:   u32,
    pub length: u32,
    pub flag:   u8,
}

#[repr(packed)]
pub struct Drives{
    pub length: u32,
    pub addr:   u32, //Указатель на массив Drive
}

pub struct Drive<T: [u16]{
    pub number:    u8,
    pub mode:      u8,
    pub cylinders: u16,
    pub heads:     u8,
    pub sectors:   u8,
    pub ports:     T,
}

#[repr(packed)]
pub struct BIOSConfigTable{
//TODO
}

#[repr(packed)]
pub struct APMTable{
    pub version:         u16,
    pub code_seg:        u16,
    pub offset:          u32,
    pub code_seg_16:     u16,
    pub data_seg:        u16,
    pub flags:           u16,
    pub code_seg_len:    u16,
    pub code_seg_16_len: u16,
    pub data_seg_len:    u16,
}

#[repr(packed)]
pub struct VBE{ 
    pub ctrl_info:     u32, // Формально, это 2 указателя на какие-то структуры. Пока не будем трогать
    pub mode_info:     u32,
    pub mode:          u16,
    pub interface_seg: u16,
    pub interface_off: u16,
    pub interface_len: u16,
}
pub struct FrameBuffer{
//TODO
}

pub static mut MULTIBOOT:*const MultiBoot=0 as *const MultiBoot;
