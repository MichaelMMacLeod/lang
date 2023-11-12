use std::{alloc::Layout, ops::Deref, ptr::NonNull};

use super::{alignment::Alignment, contains::Contains, slice::SliceBlock, split_at::TrySplitOnce};

pub struct AlignedBlock {
    block: SliceBlock,
    align: Alignment,
}

impl AlignedBlock {
    pub fn new(block: SliceBlock, align: Alignment) -> Option<Self> {
        Layout::from_size_align(block.len(), usize::from(align))
            .ok()
            .and_then(|_| {
                (block.start_ptr().align_offset(usize::from(align)) == 0)
                    .then(|| unsafe { Self::new_unchecked(block, align) })
            })
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

// b1 is aligned to 1
// 111111111
// 01234567
// offset is 01234567 - alignment = 1

// b1 is aligned to 2
// 2 2 2 2 2
// 111111111
// 01234567
// offset is 02468 - alignment = 2
// offset is 012345678 - alignment = 1

// b1 is aligned to 4
// 4   4   4
// 2 2 2 2 2
// 111111111
// 01234567
// offset is 048 - alignment = 4
// offset is 02468 - alignment = 2
// offset is 012345678 - alignment = 1

// b1 is aligned to 8
// 8       8
// 4   4   4
// 2 2 2 2 2
// 111111111
// 01234567
// offset is 08 - alignment = 8
// offset is 048 - alignment = 4
// offset is 02468 - alignment = 2
// offset is 012345678 - alignment = 1

////////

// b1 is aligned to 8
// 8       8
// 4   4   4
// 2 2 2 2 2
// 111111111
// 01234567
// offset is 08 - alignment = 8
// offset is 4 - alignment = 4
// offset is 26 - alignment = 2
// offset is 1357 - alignment = 1

// b1 is aligned to 4
// 4   4   4
// 2 2 2 2 2
// 111111111
// 01234567
// offset is 048 - alignment = 4
// offset is 26 - alignment = 2
// offset is 1357 - alignment = 1

// b1 is aligned to 2
// 2 2 2 2 2
// 111111111
// 01234567
// offset is 02468 - alignment = 2
// offset is 1357 - alignment = 1
impl TrySplitOnce for AlignedBlock {
    fn try_split_once(self, start_of_second: usize) -> Option<(Self, Self)> {
        let self_start = self.start();
        let b1_alignment = if (start_of_second + 1) <= usize::from(self.align) {
            Alignment::new(start_of_second + 1)
        } else {
            None
        };
        self.block.try_split_once(start_of_second).and_then(|(b1, b2)| {
            // safety: 'b1' should start at the same memory location as 'self'
            // does, so it must have the same alignment as 'self'.
            debug_assert_eq!(b1.start(), self_start);
            let a1 = unsafe { AlignedBlock::new_unchecked(b1, self.align) };
            todo!()
        })
    }
}