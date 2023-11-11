use std::alloc::{Layout, LayoutError};

use crate::allocation::blocks::{affixed::Affixed, block_dynamic::DynamicBlock};

use super::allocator::Allocator;

pub struct AffixedLayout {
    layout: Layout,
    middle_offset: usize,
    suffix_offset: usize,
}

pub struct IntegerOverflow;

impl From<LayoutError> for IntegerOverflow {
    fn from(_: LayoutError) -> Self {
        IntegerOverflow
    }
}

impl AffixedLayout {
    pub fn try_new(
        prefix: Layout,
        middle: Layout,
        suffix: Layout,
    ) -> Result<Self, IntegerOverflow> {
        let (layout, middle_offset) = prefix.extend(middle)?;
        let (layout, suffix_offset) = layout.extend(suffix)?;
        Ok(Self {
            layout,
            middle_offset,
            suffix_offset,
        })
    }
}

pub struct Affixer<A> {
    allocator: A,
}

pub type AffixedBlocks = Affixed<DynamicBlock, DynamicBlock, DynamicBlock, DynamicBlock>;

pub enum AffixerAllocateError<A> {
    AllocatorError(A),
    CouldNotSubdivide,
}

impl<A: Allocator<Layout, DynamicBlock>> Allocator<AffixedLayout, AffixedBlocks> for Affixer<A> {
    type AllocateError = AffixerAllocateError<A::AllocateError>;

    fn allocate(&self, layout: AffixedLayout) -> Result<AffixedBlocks, Self::AllocateError> {
        let AffixedLayout {
            layout,
            middle_offset,
            suffix_offset,
        } = layout;
        match self.allocator.allocate(layout) {
            Err(e) => Err(AffixerAllocateError::AllocatorError(e)),
            Ok(block) => block
                .try_subdivide_twice(middle_offset, suffix_offset)
                .ok_or(AffixerAllocateError::CouldNotSubdivide)
                .map(|(prefix, middle, suffix)| Affixed::new(prefix, middle, suffix, block)),
        }
    }
}
