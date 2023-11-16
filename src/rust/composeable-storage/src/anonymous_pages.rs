use std::{ffi::c_void, num::NonZeroUsize, ops::Deref};

use crate::{
    alignment::Alignment,
    units::information::{page_size_bytes, Bytes},
};

pub struct AnonymousPages {
    // We use c_void instead of u8 because we need to pass c_void to munmap, not u8.
    start_ptr: *mut c_void,
    length_bytes: Bytes<NonZeroUsize>,
}

impl AnonymousPages {
    // Invariant: 'length_bytes' must be divisible by page_size_bytes()
    pub(crate) fn new(start_ptr: *mut c_void, length_bytes: Bytes<NonZeroUsize>) -> Self {
        debug_assert!(usize::from(length_bytes.count) % usize::from(page_size_bytes().count) == 0);
        Self {
            start_ptr,
            length_bytes,
        }
    }

    pub fn start_ptr(&self) -> *mut c_void {
        self.start_ptr
    }

    pub fn length_bytes(&self) -> Bytes<NonZeroUsize> {
        self.length_bytes
    }
}

impl Deref for AnonymousPages {
    type Target = *mut c_void;

    fn deref(&self) -> &Self::Target {
        &self.start_ptr
    }
}
