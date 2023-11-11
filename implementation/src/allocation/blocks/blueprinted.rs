use std::{alloc::Layout, ops::Deref, ptr::NonNull};

use super::{aligned::AlignedBlock, contains::Contains, slice::SliceBlock};

// The current (soon to be deprecated, hopefully) global allocator API in Rust
// requires that 'alloc' receives the same layout that was used to allocate
// the pointer. But 'allocate' can return *more* memory than was requested, so
// we can't just 'dealloc' using an 'AlignedBlock'. Instead, we need to keep
// track of the length we requested originally so we can, when deallocating,
// build that original layout.
//
// I don't think this API choice makes much sense. You should be able to just
// send to 'dealloc' the actual size and alignment of the pointer. This is the
// choice being made in Rust's new Allocator API, which is good to see.
//
// The implementation of GlobalAlloc currently ignores the layout you send it
// anyway, so this really doesn't matter too much; I'm just doing it for the sake
// of correctness.
pub struct BlueprintedBlock {
    block: AlignedBlock,
    requested_len: usize,
}

impl BlueprintedBlock {
    pub fn try_new(block: AlignedBlock, requested_len: usize) -> Option<BlueprintedBlock> {
        Layout::from_size_align(requested_len, usize::from(block.align()))
            .ok()
            .map(|_| unsafe { Self::new_unchecked(block, requested_len) })
    }

    // SAFETY: The same as 'AlignedBlock', except substituting block.len() for requested_len.
    pub unsafe fn new_unchecked(block: AlignedBlock, requested_len: usize) -> BlueprintedBlock {
        Self {
            block,
            requested_len,
        }
    }

    pub fn requested_len(&self) -> usize {
        self.requested_len
    }

    pub fn requested_layout(&self) -> Layout {
        unsafe { Layout::from_size_align_unchecked(self.requested_len, usize::from(self.align())) }
    }
}

impl Deref for BlueprintedBlock {
    type Target = AlignedBlock;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl Contains<BlueprintedBlock> for BlueprintedBlock {
    fn map_part<F: FnOnce(BlueprintedBlock) -> BlueprintedBlock>(self, f: F) -> Self {
        f(self)
    }
}

impl Contains<AlignedBlock> for BlueprintedBlock {
    fn map_part<F: FnOnce(AlignedBlock) -> AlignedBlock>(self, f: F) -> Self {
        Self {
            block: f(self.block),
            ..self
        }
    }
}

impl Contains<SliceBlock> for BlueprintedBlock {
    fn map_part<F: FnOnce(SliceBlock) -> SliceBlock>(self, f: F) -> Self {
        Self {
            block: self.block.map_part(f),
            ..self
        }
    }
}
