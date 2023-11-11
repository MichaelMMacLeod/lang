use std::{ops::Deref, ptr::NonNull};

use super::{copy_as::CopyAs, start::StartBlock};

pub struct SliceBlock {
    block: NonNull<[u8]>,
}

impl SliceBlock {
    pub fn new(ptr: NonNull<[u8]>) -> Self {
        Self { block: ptr }
    }
}

impl Deref for SliceBlock {
    type Target = NonNull<[u8]>;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl CopyAs<*mut [u8]> for SliceBlock {
    fn copy_as(&self) -> *mut [u8] {
        self.as_ptr()
    }
}

impl CopyAs<*mut u8> for SliceBlock {
    fn copy_as(&self) -> *mut u8 {
        let p: *mut [u8] = self.copy_as();
        p as *mut u8
    }
}

impl CopyAs<NonNull<u8>> for SliceBlock {
    fn copy_as(&self) -> NonNull<u8> {
        unsafe { NonNull::new_unchecked(self.copy_as()) }
    }
}

impl CopyAs<StartBlock> for SliceBlock {
    fn copy_as(&self) -> StartBlock {
        let p: NonNull<u8> = self.copy_as();
        StartBlock::new(p)
    }
}
