use std::ptr::NonNull;

use super::{block::Block, block_dynamic::DynamicBlock, initialized::Initialized};

pub struct StaticBlock<const N: usize> {
    static_non_null_ptr: NonNull<[u8; N]>,
}

impl<const N: usize> StaticBlock<N> {
    fn new(static_non_null_ptr: NonNull<[u8; N]>) -> Self {
        Self { static_non_null_ptr }
    }

    pub fn static_non_null_ptr(&self) -> NonNull<[u8; N]> {
        self.static_non_null_ptr
    }

    pub fn static_ptr(&self) -> *mut [u8; N] {
        self.static_non_null_ptr().as_ptr()
    }

    pub unsafe fn initialize_with_array(self, vs: [u8; N]) -> Initialized<Self> {
        self.start_ptr().copy_from(vs.as_ptr(), N);
        Initialized::new(self)
    }
}

pub struct WrongSize;

impl<const N: usize> TryFrom<DynamicBlock> for StaticBlock<N> {
    type Error = WrongSize;

    fn try_from(value: DynamicBlock) -> Result<Self, Self::Error> {
        (value.len() == N)
            .then(|| Self::new(unsafe { NonNull::new_unchecked(value.ptr() as *mut [u8; N]) }))
            .ok_or(WrongSize)
    }
}

impl<const N: usize> Block for StaticBlock<N> {
    fn non_null_ptr(&self) -> NonNull<[u8]> {
        unsafe { NonNull::new_unchecked(self.static_non_null_ptr.as_ptr() as *mut [u8]) }
    }
}
