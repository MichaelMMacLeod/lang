use std::ptr::NonNull;

use crate::alignment::Alignment;

pub struct AlignedSlice {
    slice: NonNull<[u8]>,
    alignment: Alignment,
}

impl AlignedSlice {
    pub fn new(slice: NonNull<[u8]>, alignment: Alignment) -> Self {
        Self { slice, alignment }
    }
}
