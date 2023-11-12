use std::ptr::NonNull;

pub struct Slice {
    slice_ptr: NonNull<[u8]>,
}

impl Slice {
    pub fn new(slice_ptr: NonNull<[u8]>) -> Self {
        Self { slice_ptr }
    }
}
