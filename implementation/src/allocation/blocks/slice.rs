use std::{
    ops::Deref,
    ptr::{slice_from_raw_parts_mut, NonNull},
};

use super::{contains::Contains, initialized::Initialized, split_at::TrySplitOnce};

pub struct SliceBlock {
    block: NonNull<[u8]>,
}

impl SliceBlock {
    // SAFETY: 'ptr' must be properly aligned (i.e., it must be aligned
    // to the alignment of a 'u8', which is 1). Moreover, it must be
    // valid to write (not *read*; the slice may be uninitialized) to
    // each 'u8' in the slice. It is allowed to pass a pointer to a slice
    // of length zero to this function, although I know of no good way to
    // do this in stable Rust.
    pub unsafe fn new_unchecked(ptr: NonNull<[u8]>) -> Self {
        Self { block: ptr }
    }

    pub fn block_ptr(&self) -> *mut [u8] {
        self.as_ptr()
    }

    pub fn start_ptr(&self) -> *mut u8 {
        self.block_ptr() as *mut u8
    }

    pub fn start(&self) -> NonNull<u8> {
        unsafe { NonNull::new_unchecked(self.start_ptr()) }
    }
}

impl Deref for SliceBlock {
    type Target = NonNull<[u8]>;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl Contains<SliceBlock> for SliceBlock {
    fn map_part<F: FnOnce(SliceBlock) -> SliceBlock>(self, f: F) -> Self {
        f(self)
    }
}

impl TrySplitOnce for SliceBlock {
    fn try_split_once(self, start_of_second: usize) -> Option<(Self, Self)> {
        (start_of_second <= self.len()).then(|| {
            let second_start_ptr = unsafe { self.start_ptr().add(start_of_second) };
            let (first_ptr, second_ptr) = (
                slice_from_raw_parts_mut(self.start_ptr(), self.len()),
                slice_from_raw_parts_mut(second_start_ptr, self.len() - start_of_second),
            );
            let (first_non_null_ptr, second_non_null_ptr) = unsafe {
                (
                    NonNull::new_unchecked(first_ptr),
                    NonNull::new_unchecked(second_ptr),
                )
            };
            unsafe {
                (
                    Self::new_unchecked(first_non_null_ptr),
                    Self::new_unchecked(second_non_null_ptr),
                )
            }
        })
    }
}
