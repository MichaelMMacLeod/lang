use std::{alloc::Layout, ops::Deref, ptr::NonNull};

use super::{alignment::Alignment, contains::Contains, slice::SliceBlock};

pub struct AlignedBlock {
    block: SliceBlock,
    align: Alignment,
}

impl AlignedBlock {
    pub fn new(block: SliceBlock, align: Alignment) -> Option<Self> {
        Layout::from_size_align(block.len(), usize::from(align))
            .ok()
            .map(|_| unsafe { Self::new_unchecked(block, align) })
    }

    // SAFETY: 'block' must be allocated with alignment 'align'. Moreover, 'block.len()',
    // when rounded up to the nearest multiple of 'align', must be less than or equal to
    // isize::MAX.
    pub unsafe fn new_unchecked(block: SliceBlock, align: Alignment) -> Self {
        Self { block, align }
    }

    pub fn align(&self) -> Alignment {
        self.align
    }

    pub fn allocated_layout(&self) -> Layout {
        unsafe { Layout::from_size_align_unchecked(self.len(), usize::from(self.align())) }
    }
}

impl Deref for AlignedBlock {
    type Target = SliceBlock;

    fn deref(&self) -> &Self::Target {
        &self.block
    }
}

impl Contains<AlignedBlock> for AlignedBlock {
    fn map_part<F: FnOnce(AlignedBlock) -> AlignedBlock>(self, f: F) -> Self {
        f(self)
    }
}

impl Contains<SliceBlock> for AlignedBlock {
    fn map_part<F: FnOnce(SliceBlock) -> SliceBlock>(self, f: F) -> Self {
        Self {
            block: f(self.block),
            ..self
        }
    }
}
