#[repr(packed)]
pub struct MultiBoot {
    flags: u32,
    pub mem: LUMem,
    pub boot: BootDevice,
    pub cmdline: *const u8,
    pub mods: Modules,
    pub syms: SymbolTable,
    pub mmap: Mmap,
    pub drives: Drives,
    pub config: &'static BIOSConfigTable,
    pub loader: *const u8,
    pub apm: &'static APMTable,
    pub vbe: VBE,
    pub fb: FrameBuffer,
}
impl MultiBoot {
    pub fn flags(&self) -> u32 {
        self.flags
    }
}
#[repr(packed)]
pub struct LUMem {
    pub lower: u32,
    pub upper: u32,
}
#[repr(packed)]
pub struct BootDevice {
    pub parts: [u8; 4],
}
#[repr(packed)]
pub struct Modules {
    pub count: u32,
    pub addr: u32,
    // TODO: заглушка
}
#[repr(packed)]
pub struct SymbolTable {
    pub num: [u32; 4],
}
#[repr(packed)]
pub struct Mmap {
    pub length: usize,
    pub addr: *const Frame, //Указатель на массив Frame
}
pub struct Frame {
    pub size: u32,
    pub addr: u32,
    _addr_high: u32,
    pub length: u32,
    _len_high: u32,
    pub flag: u32,
}
#[repr(packed)]
pub struct Drives {
    pub length: u32,
    pub addr: u32, //Указатель на массив Drive
}
pub struct Drive<T> {
    pub number: u8,
    pub mode: u8,
    pub cylinders: u16,
    pub heads: u8,
    pub sectors: u8,
    pub ports: T,
}
#[repr(packed)]
pub struct BIOSConfigTable {
    //TODO
}
#[repr(packed)]
pub struct APMTable {
    pub version: u16,
    pub code_seg: u16,
    pub offset: u32,
    pub code_seg_16: u16,
    pub data_seg: u16,
    pub flags: u16,
    pub code_seg_len: u16,
    pub code_seg_16_len: u16,
    pub data_seg_len: u16,
}
#[repr(packed)]
pub struct VBE {
    pub ctrl_info: u32, // Формально, это 2 указателя на какие-то структуры. Пока не будем трогать
    pub mode_info: u32,
    pub mode: u16,
    pub interface_seg: u16,
    pub interface_off: u16,
    pub interface_len: u16,
}
#[repr(packed)]
pub struct FrameBuffer {
    pub addr: u32,
    pub _addr: u32,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub flag: u8, // TODO
}
