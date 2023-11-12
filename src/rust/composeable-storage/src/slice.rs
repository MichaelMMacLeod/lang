use std::ptr::NonNull;

use crate::alignment::Alignment;

pub struct Slice {
    slice: NonNull<[u8]>,
    alignment: Alignment,
}

impl Slice {
    pub fn new(slice: NonNull<[u8]>, alignment: Alignment) -> Self {
        Self { slice, alignment }
    }
}
