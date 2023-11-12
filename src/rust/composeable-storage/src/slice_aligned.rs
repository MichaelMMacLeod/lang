use crate::{alignment::Alignment, slice::Slice};

pub struct SliceAligned {
    slice: Slice,
    alignment: Alignment,
}

impl SliceAligned {
    pub fn new(slice: Slice, alignment: Alignment) -> Self {
        Self { slice, alignment }
    }
}
