// use std::num::NonZeroUsize;

// use super::{
//     alignment::{self, Alignment},
//     blocks::{Block, DynamicSizedBlock, StaticSizedBlock},
// };

// pub trait AllocBase {
//     type DynamicAllocFail;
//     type StaticAllocFail;
//     fn good_size_for(num_bytes: usize) -> usize;
// }

// pub trait TryAlloc: AllocBase {
//     fn try_allocate_dynamic(
//         &mut self,
//         alignment: Alignment,
//         num_bytes: usize,
//     ) -> Result<DynamicSizedBlock, Self::DynamicAllocFail>;
//     fn try_allocate_static<const NUM_BYTES: usize>(
//         &mut self,
//         alignment: Alignment,
//     ) -> Result<StaticSizedBlock<NUM_BYTES>, Self::StaticAllocFail>;
// }

// pub trait Alloc: AllocBase {
//     fn allocate_dynamic(&mut self, alignment: Alignment, num_bytes: usize) -> DynamicSizedBlock;
//     fn allocate_static<const NUM_BYTES: usize>(
//         &mut self,
//         alignment: Alignment,
//     ) -> StaticSizedBlock<NUM_BYTES>;
// }

// pub trait GetAlloc<B: Block>: TryAlloc {
//     type GetFail;
//     fn get(&self, block: &B) -> Result<&[u8], Self::GetFail>;
//     fn get_mut(&mut self, block: &B) -> Result<&mut [u8], Self::GetFail>;
// }

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
