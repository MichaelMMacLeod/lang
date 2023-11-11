use std::{ptr::NonNull, ops::Deref};

use super::copy_as::CopyAs;

pub struct StartBlock {
    start: NonNull<u8>,
}

impl StartBlock {
    pub fn new(start: NonNull<u8>) -> Self {
        Self { start }
    }
}

impl Deref for StartBlock {
    type Target = NonNull<u8>;

    fn deref(&self) -> &Self::Target {
        &self.start
    }
}

impl CopyAs<*mut u8> for StartBlock {
    fn copy_as(&self) -> *mut u8 {
        self.as_ptr()
    }
}