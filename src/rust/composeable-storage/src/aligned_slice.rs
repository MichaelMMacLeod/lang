use std::{ops::Deref, ptr::NonNull, u8};

use crate::alignment::Alignment;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AlignedSlice {
    slice: NonNull<[u8]>,
    alignment: Alignment,
}

impl AlignedSlice {
    pub fn new(slice: NonNull<[u8]>, alignment: Alignment) -> Self {
        Self { slice, alignment }
    }

    pub fn alignment(&self) -> Alignment {
        self.alignment
    }

    pub fn slice_ptr(&self) -> *mut [u8] {
        self.slice.as_ptr()
    }

    pub fn start_ptr(&self) -> *mut u8 {
        self.as_ptr() as *mut u8
    }
}

impl Deref for AlignedSlice {
    type Target = NonNull<[u8]>;

    fn deref(&self) -> &Self::Target {
        &self.slice
    }
}
