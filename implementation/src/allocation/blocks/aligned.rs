// use std::{alloc::Layout, ptr::NonNull};

// use super::{alignment::Alignment, slice::SliceBlock};

// pub struct AlignedBlock<B: Into<SliceBlock>> {
//     // invariant: Layout::from_size_align(block.len(), usize::from(align)).is_ok()
//     block: B,
//     align: Alignment,
// }

// impl<B: Into<SliceBlock>> AlignedBlock<B> {
//     pub fn try_new(block: B, align: Alignment) -> Option<Self> {
//         Layout::from_size_align(block.into().len(), usize::from(align))
//             .map(|_| Self { block, align })
//             .ok()
//     }

//     pub fn align(&self) -> Alignment {
//         self.align
//     }

//     // pub fn layout(&self) -> Layout {
//     //     // SAFETY: we checked that the non-unchecked version was .is_ok()
//     //     // in try_new(), moreover, we never mutate the insides, so that
//     //     // should still hold.
//     //     unsafe { Layout::from_size_align_unchecked(self.block.len(), usize::from(self.align)) }
//     // }

//     // pub fn len(&self) -> usize {
//     //     self.block.len()
//     // }

//     // pub fn block(&self) -> NonNull<[u8]> {
//     //     self.block
//     // }

//     // pub fn block_ptr(&self) -> *mut [u8] {
//     //     self.block.as_ptr()
//     // }

//     // pub fn start(&self) -> NonNull<u8> {
//     //     unsafe { NonNull::new_unchecked(self.start_ptr()) }
//     // }

//     // pub fn start_ptr(&self) -> *mut u8 {
//     //     self.block_ptr() as *mut u8
//     // }
// }
