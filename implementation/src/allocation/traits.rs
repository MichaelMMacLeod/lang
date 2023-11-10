use std::{
    alloc::{Layout, LayoutError},
    ptr::NonNull,
};

use super::block::{Block, DisjointBlocks};

pub trait GoodSize {
    fn good_size_for(&self, layout: Layout) -> usize;
}

// type AllocFunction<A> = ;

pub trait Alloc {
    type AllocError;

    fn alloc_uninitialized(&self, layout: Layout) -> Result<Block, Self::AllocError>;
}

pub trait ZeroingAlloc: Alloc {
    fn alloc_zeroed(&self, layout: Layout) -> Result<Block, Self::AllocError> {
        let blk = self.alloc_uninitialized(layout)?;
        let ptr: *mut u8 = blk.into();
        unsafe {
            ptr.write_bytes(0, NonNull::<[u8]>::from(blk).len());
        }
        Ok(blk)
    }
}
impl<A: Alloc> ZeroingAlloc for A {}

#[derive(Debug)]
pub enum AffixAllocError<E> {
    LayoutError(LayoutError),
    CouldNotSplit,
    AllocError(E),
}

impl<E> From<LayoutError> for AffixAllocError<E> {
    fn from(value: LayoutError) -> Self {
        Self::LayoutError(value)
    }
}

pub fn alloc_affixed<E, F: FnMut(Layout) -> Result<Block, E>>(
    mut alloc: F,
    prefix: Layout,
    middle: Layout,
    suffix: Layout,
) -> Result<DisjointBlocks<3>, AffixAllocError<E>> {
    let (combined_layout, middle_offset) = prefix.extend(middle)?;
    let (combined_layout, suffix_offset) = combined_layout.extend(suffix)?;
    match alloc(combined_layout) {
        Err(e) => Err(AffixAllocError::AllocError(e)),
        Ok(blk) => Ok(blk
            .split3(middle_offset, suffix_offset)
            .ok_or(AffixAllocError::CouldNotSplit)?),
    }
}

pub trait Dealloc {
    type DeallocError;

    unsafe fn dealloc(&self, block: Block) -> Option<Self::DeallocError>;
}

pub trait Owns {
    fn owns(&self, block: Block) -> bool;
}
