use std::ptr::{slice_from_raw_parts_mut, NonNull};

use super::initialized::Initialized;

pub struct DynamicBlock {
    non_null_ptr: NonNull<[u8]>,
}

impl DynamicBlock {
    pub fn new(non_null_ptr: NonNull<[u8]>) -> Self {
        Self { non_null_ptr }
    }

    pub unsafe fn initialize_with_constant(self, c: u8) -> Initialized<Self> {
        self.start_ptr().write_bytes(c, self.len());
        Initialized::new(self)
    }

    pub fn try_subdivide_once(&self, start_of_second: usize) -> Option<(Self, Self)> {
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
            (
                Self::new(first_non_null_ptr),
                Self::new(second_non_null_ptr),
            )
        })
    }

    pub fn try_subdivide_twice(
        &self,
        start_of_second: usize,
        start_of_third: usize,
    ) -> Option<(Self, Self, Self)> {
        let third_offset = start_of_third.checked_sub(start_of_second)?;
        let (prefix, rest) = self.try_subdivide_once(start_of_second)?;
        let (middle, suffix) = rest.try_subdivide_once(third_offset)?;
        Some((prefix, middle, suffix))
    }

    pub fn len(&self) -> usize {
        self.non_null_ptr.len()
    }

    pub fn aligned_to(&self, align: usize) -> bool {
        self.start_ptr().align_offset(align) == 0
    }

    pub(super) fn ptr(&self) -> *mut [u8] {
        self.non_null_ptr.as_ptr()
    }

    pub(super) fn start_ptr(&self) -> *mut u8 {
        self.ptr() as *mut u8
    }
}
