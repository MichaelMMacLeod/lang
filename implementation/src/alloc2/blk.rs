use std::{alloc::Layout, ptr::NonNull};

#[derive(Clone, Copy)]
pub struct Blk {
    ptr: NonNull<[u8]>,

    // This is the layout that the Blk was allocated using. WARNING; DANGER:
    // this may be smaller than the length of 'ptr' given that allocate may
    // return a larger memory region than requested.
    blueprint: Layout,
}

impl Blk {
    pub fn new(ptr: NonNull<[u8]>, blueprint: Layout) -> Self {
        Self { ptr, blueprint }
    }
}

impl Blk {
    pub fn blueprint(&self) -> Layout {
        self.blueprint
    }
}

impl From<Blk> for NonNull<[u8]> {
    fn from(value: Blk) -> Self {
        value.ptr
    }
}

impl From<Blk> for *mut [u8] {
    fn from(value: Blk) -> Self {
        value.ptr.as_ptr()
    }
}

impl From<Blk> for *mut u8 {
    fn from(value: Blk) -> Self {
        let ptr: *mut [u8] = value.into();
        ptr as *mut u8
    }
}

impl From<Blk> for NonNull<u8> {
    fn from(value: Blk) -> Self {
        let ptr: *mut u8 = value.into();
        unsafe { NonNull::new_unchecked(ptr) }
    }
}
