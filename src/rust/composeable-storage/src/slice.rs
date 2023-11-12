use std::{ptr::NonNull, ops::Deref, u8};

use crate::alignment::Alignment;

pub struct Ram {
    slice: NonNull<[u8]>,
    alignment: Alignment,
}

impl Ram {
    pub fn new(slice: NonNull<[u8]>, alignment: Alignment) -> Self {
        Self { slice, alignment }
    }

    pub fn alignment(&self) -> Alignment {
        self.alignment
    }

    pub fn start_ptr(&self) -> *mut u8 {
        self.as_ptr() as *mut u8
    }
}

impl Deref for Ram {
    type Target = NonNull<[u8]>;

    fn deref(&self) -> &Self::Target {
        &self.slice
    }
}