use std::alloc::Layout;

use crate::slice::Slice;

pub struct GlobalAllocSlice {
    slice: Slice,
    layout: Layout,
}

impl GlobalAllocSlice {
    pub fn new(slice: Slice, blueprint: Layout) -> Self {
        Self { slice, layout: blueprint }
    }
}