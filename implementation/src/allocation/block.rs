// use std::{
//     alloc::Layout,
//     ptr::{slice_from_raw_parts_mut, NonNull},
// };

// pub trait BlockTrait: AsMut<NonNull<[u8]>> + AsRef<NonNull<[u8]>> {}

// #[derive(Clone, Copy)]
// pub struct Block {
//     ptr: NonNull<[u8]>,

//     // This is the layout that the Blk was allocated using. WARNING; DANGER:
//     // this may be smaller than the length of 'ptr' given that allocate may
//     // return a larger memory region than requested.
//     blueprint: Option<Layout>,
// }

// impl Block {
//     pub fn new(ptr: NonNull<[u8]>, blueprint: Option<Layout>) -> Self {
//         Self { ptr, blueprint }
//     }

//     pub fn len(&self) -> usize {
//         let ptr: NonNull<[u8]> = (*self).into();
//         ptr.len()
//     }

//     // Splits self into two blocks, one which contains the first 'middle' number of 'u8's,
//     // and one which contains the rest (if any). If 'middle' is more than the number of
//     // 'u8's pointed to by self, returns None.
//     pub fn split2(&self, middle: usize) -> Option<DisjointBlocks<2>> {
//         let ptr: NonNull<[u8]> = (*self).into();
//         let length = ptr.len();
//         if middle <= length {
//             let ptr: *mut u8 = (*self).into();
//             let tail = unsafe { ptr.add(middle) };
//             let (p1, p2) = (
//                 slice_from_raw_parts_mut(ptr, middle),
//                 slice_from_raw_parts_mut(tail, length - middle),
//             );
//             let (p1, p2) = unsafe { (NonNull::new_unchecked(p1), NonNull::new_unchecked(p2)) };
//             Some(DisjointBlocks {
//                 blocks: [Block::new(p1, None), Block::new(p2, None)],
//             })
//         } else {
//             None
//         }
//     }

//     // Splits self into three blocks. The first contains the first 'offset1' number of 'u8's
//     // pointed to by 'self'. The second contains the next 'offset2 - offset1' number of 'u8's.
//     // The last contains the rest of the 'u8's pointed to by self.
//     pub fn split3(&self, offset1: usize, offset2: usize) -> Option<DisjointBlocks<3>> {
//         let middle2 = offset2.checked_sub(offset1)?;
//         let first_split2 = self.split2(offset1)?;
//         let second_split2 = first_split2.blocks()[1].split2(middle2)?;
//         Some(DisjointBlocks {
//             blocks: [
//                 first_split2.blocks()[0],
//                 second_split2.blocks()[0],
//                 second_split2.blocks()[1],
//             ],
//         })
//     }

//     pub unsafe fn as_ref(&self) -> &[u8] {
//         let ptr: NonNull<[u8]> = (*self).into();
//         ptr.as_ref()
//     }

//     pub unsafe fn as_mut(&self) -> &mut [u8] {
//         let mut ptr: NonNull<[u8]> = (*self).into();
//         ptr.as_mut()
//     }
// }

// impl Block {
//     pub fn blueprint(&self) -> Option<Layout> {
//         self.blueprint
//     }
// }

// impl From<Block> for NonNull<[u8]> {
//     fn from(value: Block) -> Self {
//         value.ptr
//     }
// }

// impl From<Block> for *mut [u8] {
//     fn from(value: Block) -> Self {
//         value.ptr.as_ptr()
//     }
// }

// impl From<Block> for *const [u8] {
//     fn from(value: Block) -> Self {
//         value.ptr.as_ptr()
//     }
// }

// impl From<Block> for *const u8 {
//     fn from(value: Block) -> Self {
//         let ptr: *const [u8] = value.into();
//         ptr as *const u8
//     }
// }

// impl From<Block> for *mut u8 {
//     fn from(value: Block) -> Self {
//         let ptr: *mut [u8] = value.into();
//         ptr as *mut u8
//     }
// }

// impl From<Block> for NonNull<u8> {
//     fn from(value: Block) -> Self {
//         let ptr: *mut u8 = value.into();
//         unsafe { NonNull::new_unchecked(ptr) }
//     }
// }

// pub struct DisjointBlocks<const N: usize> {
//     blocks: [Block; N],
// }

// impl<const N: usize> DisjointBlocks<N> {
//     pub fn blocks(&self) -> &[Block; N] {
//         &self.blocks
//     }
// }