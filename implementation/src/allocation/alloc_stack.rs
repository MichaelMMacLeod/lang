// use bitvec::vec::BitVec;

// use super::{traits_alloc::TryAlloc, blocks::DynamicSizedBlock};

// pub struct StackAllocator<const MAX_BYTES: usize> {
//     buffer: BitVec,
//     top: usize,
//     ceiling: usize,
// }

// struct HitCeiling;

// impl<const MAX_BYTES: usize> TryAlloc<DynamicSizedBlock> for StackAllocator<MAX_BYTES> {
//     type DynamicAllocFail = HitCeiling;

//     fn allocate_dynamic(&mut self, num_bits: usize) -> Result<B, Self::DynamicAllocFail> {
//         todo!()
//     }
// }

// // use super::traits::{Alloc, Block, GetAlloc, InfallibleAlloc};

// // pub struct BoxAllocator {
// //     boxes: Vec<Box<[u8]>>,
// // }

// // impl InfallibleAlloc for BoxAllocator {
// //     fn allocate_always(&mut self, size: usize) -> super::traits::Block {
// //         let block = Block::new(self.boxes.len(), size);
// //         self.boxes.push(Box::from_iter((0..=0).cycle().take(size)));
// //         block
// //     }
// // }

// // pub struct WrongBlock;

// // impl GetAlloc for BoxAllocator {
// //     type GetFail = WrongBlock;

// //     fn get_mut(&mut self, block: &Block) -> Result<&mut [u8], Self::GetFail> {
// //         self.boxes
// //             .get_mut(*block.key())
// //             .map(|b| b.as_mut())
// //             .ok_or(WrongBlock)
// //     }

// //     fn get(&self, block: &Block) -> Result<&[u8], Self::GetFail> {
// //         self.boxes
// //             .get(*block.key())
// //             .map(|b| b.as_ref())
// //             .ok_or(WrongBlock)
// //     }
// // }
