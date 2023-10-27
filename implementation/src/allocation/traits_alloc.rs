use std::num::NonZeroUsize;

use bitvec::{
    prelude::BitArray,
    view::{BitView, BitViewSized},
};

use super::{
    alignment::{self, Alignment},
    blocks::{Block, DynamicSizedBlock, StaticSizedBlock},
};

pub trait GoodSizeAlloc {
    fn good_size_for(num_bytes: usize) -> usize;
}

pub trait AllocStatic {
    fn allocate_static<const NUM_BYTES: usize>(
        &mut self,
        alignment: Alignment,
    ) -> StaticSizedBlock<NUM_BYTES>;
}

pub trait AllocDynamic {
    fn allocate_dynamic(&mut self, alignment: Alignment, num_bytes: usize) -> DynamicSizedBlock;
}

pub trait TryAllocStatic {
    type StaticAllocFail;
    fn try_allocate_static<const NUM_BYTES: usize>(
        &mut self,
        alignment: Alignment,
    ) -> Result<StaticSizedBlock<NUM_BYTES>, Self::StaticAllocFail>;
}

pub trait TryAllocDynamic {
    type DynamicAllocFail;
    fn try_allocate_dynamic(
        &mut self,
        alignment: Alignment,
        num_bytes: usize,
    ) -> Result<DynamicSizedBlock, Self::DynamicAllocFail>;
}

pub trait GetAllocStatic<const NUM_BYTES: usize> {
    type GetFail;
    fn try_get(
        &self,
        block: &StaticSizedBlock<NUM_BYTES>,
    ) -> Result<&BitArray<[usize; NUM_BYTES]>, Self::GetFail>;
    fn try_get_mut(
        &self,
        block: &StaticSizedBlock<NUM_BYTES>,
    ) -> Result<&mut BitArray<[usize; NUM_BYTES]>, Self::GetFail>;
}

// pub trait OwnsAlloc<B: Block>: TryAlloc<B> {
//     fn owns(&self, block: &B) -> bool;
// }

// impl<B: Block, T: GetAlloc<B>> OwnsAlloc<B> for T {
//     fn owns(&self, block: &B) -> bool {
//         self.get(block).is_ok()
//     }
// }

// pub trait Dealloc<B: Block>: TryAlloc<B> {
//     type DeallocFail;
//     fn deallocate(&mut self, block: &B) -> Option<Self::DeallocFail>;
// }

// pub trait InfallibleDealloc<B: Block>: TryAlloc<B> {
//     fn deallocate_always(&mut self, block: &B);
// }

// impl<B: Block, T: InfallibleDealloc<B>> Dealloc<B> for T {
//     type DeallocFail = bool;
//     fn deallocate(&mut self, block: &B) -> Option<Self::DeallocFail> {
//         self.deallocate_always(block);
//         None
//     }
// }

// pub trait AllocAll<B: Block>: TryAlloc<B> {
//     type AllocAllFail;
//     fn allocate_all(&mut self) -> Result<B, Self::DynamicAllocFail>;
// }

// pub trait DeallocAll<B: Block>: TryAlloc<B> {
//     type DeallocAllFail;
//     fn dealloc_all(&mut self) -> Option<Self::DeallocAllFail>;
// }

// pub trait GrowAlloc<B: Block> {
//     type GrowAllocFail;
//     fn grow(&mut self, block: B, bytes_to_add: usize) -> Result<B, Self::GrowAllocFail>;
// }

// pub trait ShrinkAlloc<B: Block> {
//     type ShrinkAllocFail;
//     fn shrink(&mut self, block: B, bytes_to_remove: usize) -> Result<B, Self::ShrinkAllocFail>;
// }

// impl<T: Alloc> TryAlloc for T {
//     type DynamicAllocFail = Self::DynamicAllocFail;
//     type StaticAllocFail = Self::StaticAllocFail;
//     fn try_allocate_dynamic(
//         &mut self,
//         alignment: Alignment,
//         num_bytes: usize,
//     ) -> Result<DynamicSizedBlock, Self::DynamicAllocFail> {
//         Ok(self.allocate_dynamic(alignment, num_bytes))
//     }
//     fn try_allocate_static<const NUM_BYTES: usize>(
//         &mut self,
//         alignment: Alignment,
//     ) -> Result<StaticSizedBlock<NUM_BYTES>, Self::StaticAllocFail> {
//         Ok(self.allocate_static(alignment))
//     }
// }
