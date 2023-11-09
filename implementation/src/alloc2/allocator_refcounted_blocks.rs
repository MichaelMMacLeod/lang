// use typenum::Unsigned;

// use super::{
//     allocator_box::{BoxAllocator, BoxBlock},
//     traits::{Alloc, GetMut},
// };

// pub struct BookkeeperBlock {
//     index: usize,
// }

// pub struct Bookkeeper<M> {
//     block_size: usize,
//     blocks: Vec<u8>,
//     metadata: M,
// }

// impl<M> Bookkeeper<M> {
//     fn check_invariants(&self) {
//         assert!(self.metadata.len() * self.block_size == self.blocks.len());
//     }

//     pub fn new(block_size: usize, block_count: usize) -> Self {
//         let ret = Self {
//             block_size,
//             blocks: vec![0; block_size * block_count],
//             metadata: Vec::with_capacity(block_count),
//         };
//         ret.check_invariants();
//         ret
//     }
// }

// pub struct RefCount(usize);

// impl Alloc for Bookkeeper<RefCount> {
//     type Block;

//     type AllocError;

//     fn alloc(&mut self, layout: std::alloc::Layout) -> Result<Self::Block, Self::AllocError> {
//         todo!()
//     }
// }

