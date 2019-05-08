#![no_std]
#![feature(unsize)]
#![feature(coerce_unsized)]
extern crate spin;
#[macro_use]
extern crate devices;
extern crate multiboot;
use spin::Mutex;
pub fn init(mb_mmap: &Option<&'static [multiboot::Frame]>, cs: u32, be: u32) {
    if let Some(mmap) = mb_mmap {
        for i in 0..mmap.len() {
            if mmap[i].flag == 1 {
                let mut reg_start = mmap[i].addr;
                let mut reg_end = mmap[i].addr + mmap[i].length;
                if reg_start == cs {
                    reg_start = be;
                }
                unsafe {
                    HEAP.lock()
                        .add_region(reg_start as usize, (reg_end - reg_start) as usize)
                };
            }
        }
    }
}
pub struct Heap {
    area: *mut Area,
}
struct Area {
    size: usize,
    next: *mut Area,
}
// All raw pointers to unknown type are interpreted as *const u32 or &u32
const MIN_SIZE: usize = core::mem::size_of::<Area>();
const MIN_ALIGN: usize = core::mem::align_of::<Area>(); // I think, MIN_SIZE > MIN_ALIGN. But if MIN_SIZE<MIN_ALIGN?
impl Heap {
    fn dealloc(&mut self, pointer: *mut Area, mut size: usize) {
        size = align_fn(size, MIN_ALIGN);
        unsafe {
            (*pointer).next = self.area;
            (*pointer).size = size;
        }
        self.area = pointer;
    }
    fn alloc(&mut self, mut size: usize, mut align: usize) -> *mut u32 {
        if align > 20 {
            return 0 as *mut u32;
        }
        let mut area: *mut *mut Area = if (self.area as usize) == 0 {
            panic!("Heap not inited!");
        } else {
            &mut self.area
        };
        align = 1 << align;
        align = core::cmp::max(align, MIN_ALIGN);
        size = core::cmp::max(size, MIN_SIZE);
        size = align_fn(size, align);
        loop {
            let start;
            let end;
            unsafe {
                start = *area as usize;
                end = (**area).size + start;
            }
            let mut rstart = align_fn(start, align);
            if rstart != start {
                rstart = align_fn(start + MIN_SIZE, align);
            }
            if rstart + size > end {
                unsafe {
                    area = if ((**area).next as usize) == 0 {
                        panic!("Unknown error");
                        return 0 as *mut u32;
                    } else {
                        &mut (**area).next
                    };
                }
            } else {
                if rstart > start {
                    unsafe {
                        (**area).size = rstart - start;
                    }
                } else {
                    unsafe {
                        *area = (**area).next;
                    }
                };
                if rstart + size < end {
                    self.dealloc((rstart + size) as *mut Area, end - rstart - size);
                }
                return rstart as *mut u32;
            }
        }
    }
    pub unsafe fn add_region(&mut self, mut addr: usize, mut size: usize) {
        size = (size / MIN_SIZE) * MIN_SIZE;
        if size < MIN_SIZE {
            return;
        }
        addr = align_fn(addr, MIN_ALIGN);
        self.dealloc(addr as *mut Area, size);
    }
}
unsafe impl core::marker::Send for Heap {}
pub static HEAP: Mutex<Heap> = Mutex::new(Heap {
    area: 0 as *mut Area,
});

pub struct Owned<T: 'static + ?Sized> {
    pointer: &'static mut T,
}
impl<T> Owned<T> {
    pub fn new(x: T) -> Owned<T> {
        let addr = HEAP
            .lock()
            .alloc(core::mem::size_of_val(&x), core::mem::align_of_val(&x))
            as *mut T;
        if addr as u32 == 0 {
            panic!();
        }
        unsafe {
            core::ptr::copy(&x, addr, 1);
            core::mem::forget(x);
            Owned {
                pointer: &mut *addr,
            }
        }
    }
}
impl<T: Copy + ?Sized> Owned<T> {
    pub fn new_copy(x: &T) -> Owned<T> {
        let addr = HEAP
            .lock()
            .alloc(core::mem::size_of_val(&x), core::mem::align_of_val(&x))
            as *mut T;
        if addr as u32 == 0 {
            panic!();
        }
        unsafe {
            *addr = *x;
            Owned {
                pointer: &mut *addr,
            }
        }
    }
}
impl<T: Default> Owned<[T]> {
    pub fn new_array(count: usize) -> Owned<[T]> {
        let addr = HEAP.lock().alloc(
            core::mem::size_of::<T>() * count,
            core::mem::align_of::<T>(),
        ) as *mut T;
        if addr as u32 == 0 {
            panic!();
        }
        unsafe {
            let pointer = core::slice::from_raw_parts_mut(addr as *mut T, count);
            for i in 0..pointer.len() {
                pointer[i] = Default::default();
            }
            Owned { pointer: pointer }
        }
    }
}
impl<T: ?Sized> Drop for Owned<T> {
    fn drop(&mut self) {
        unsafe {
            core::ptr::drop_in_place(self.pointer);
            HEAP.lock().dealloc(
                self.pointer as *mut T as *mut Area,
                core::mem::size_of_val(self.pointer),
            ); // May be size of val eq 0? TODO
        }
    }
}
impl<T: ?Sized + core::marker::Unsize<U>, U: ?Sized> core::ops::CoerceUnsized<Owned<U>>
    for Owned<T>
{
}
impl<T: ?Sized> core::ops::Deref for Owned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.pointer
    }
}
impl<T: ?Sized> core::ops::DerefMut for Owned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.pointer
    }
}
unsafe impl<T: ?Sized> core::marker::Send for Owned<T> {}
fn align_fn(size: usize, align: usize) -> usize {
    (size + align - 1) & !(align - 1)
}
