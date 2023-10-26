use std::num::NonZeroUsize;

use super::blocks::Block;

// Some simple composable traits for allocators that return
// blocks of u8 slices

pub trait Alloc<B: Block> {
    type AllocFail;
    const GOOD_SIZE: Option<NonZeroUsize>;
    const ALIGNMENT: usize;
    fn allocate(&mut self, num_bits: usize) -> Result<B, Self::AllocFail>;
}

pub trait InfallibleAlloc<B: Block> {
    const ALIGNMENT: usize;
    fn allocate_always(&mut self, num_bits: usize) -> B;
}

impl<B: Block, T: InfallibleAlloc<B>> Alloc<B> for T {
    type AllocFail = bool /* never constructed */;

    const GOOD_SIZE: Option<NonZeroUsize> = None;
    const ALIGNMENT: usize = Self::ALIGNMENT;

    fn allocate(&mut self, size: usize) -> Result<B, Self::AllocFail> {
        Ok(self.allocate_always(size))
    }
}

pub trait GetAlloc<B: Block>: Alloc<B> {
    type GetFail;
    fn get(&self, block: &B) -> Result<&[u8], Self::GetFail>;
    fn get_mut(&mut self, block: &B) -> Result<&mut [u8], Self::GetFail>;
}

pub trait OwnsAlloc<B: Block>: Alloc<B> {
    fn owns(&self, block: &B) -> bool;
}

impl<B: Block, T: GetAlloc<B>> OwnsAlloc<B> for T {
    fn owns(&self, block: &B) -> bool {
        self.get(block).is_ok()
    }
}

pub trait Dealloc<B: Block>: Alloc<B> {
    type DeallocFail;
    fn deallocate(&mut self, block: &B) -> Option<Self::DeallocFail>;
}

pub trait InfallibleDealloc<B: Block>: Alloc<B> {
    fn deallocate_always(&mut self, block: &B);
}

impl<B: Block, T: InfallibleDealloc<B>> Dealloc<B> for T {
    type DeallocFail = bool /* never constructed */;

    fn deallocate(&mut self, block: &B) -> Option<Self::DeallocFail> {
        self.deallocate_always(block);
        None
    }
}

pub trait AllocAll<B: Block>: Alloc<B> {
    type AllocAllFail;
    fn allocate_all(&mut self) -> Result<B, Self::AllocFail>;
}

pub trait DeallocAll<B: Block>: Alloc<B> {
    type DeallocAllFail;
    fn dealloc_all(&mut self) -> Option<Self::DeallocAllFail>;
}

pub trait GrowAlloc<B: Block> {
    type GrowAllocFail;
    fn grow(&mut self, block: B, bytes_to_add: usize) -> Result<B, Self::GrowAllocFail>;
}

pub trait ShrinkAlloc<B: Block> {
    type ShrinkAllocFail;
    fn shrink(&mut self, block: B, bytes_to_remove: usize) -> Result<B, Self::ShrinkAllocFail>;
}