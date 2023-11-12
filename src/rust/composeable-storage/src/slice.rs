use std::{ptr::NonNull, ops::Deref, u8};

use crate::alignment::Alignment;

pub struct Slice {
    slice: NonNull<[u8]>,
    alignment: Alignment,
}

impl Slice {
    pub fn new(slice: NonNull<[u8]>, alignment: Alignment) -> Self {
        Self { slice, alignment }
    }

    pub fn slice_ptr(&self) -> *mut [u8] {
        self.as_ptr()
    }

    pub fn start_ptr(&self) -> *mut u8 {
        self.slice_ptr() as *mut u8
    }
}

impl Deref for Slice {
    type Target = NonNull<[u8]>;

    fn deref(&self) -> &Self::Target {
        &self.slice
    }
}