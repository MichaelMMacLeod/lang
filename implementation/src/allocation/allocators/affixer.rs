// use std::alloc::{Layout, LayoutError};

// use crate::allocation::blocks::{affixed::Affixed, block_dynamic::DynamicBlock, blueprinted::Blueprinted};

// use super::allocator::Allocator;

// pub struct AffixedLayout {
//     layout: Layout,
//     middle_offset: usize,
//     suffix_offset: usize,
// }

// #[derive(Debug)]
// pub struct IntegerOverflow;

// impl From<LayoutError> for IntegerOverflow {
//     fn from(_: LayoutError) -> Self {
//         IntegerOverflow
//     }
// }

// impl AffixedLayout {
//     pub fn try_new(
//         prefix: Layout,
//         middle: Layout,
//         suffix: Layout,
//     ) -> Result<Self, IntegerOverflow> {
//         let (layout, middle_offset) = prefix.extend(middle)?;
//         let (layout, suffix_offset) = layout.extend(suffix)?;
//         Ok(Self {
//             layout,
//             middle_offset,
//             suffix_offset,
//         })
//     }
// }

// pub struct Affixer<A> {
//     allocator: A,
// }

// impl<A> Affixer<A> {
//     pub fn new(allocator: A) -> Self { Self { allocator } }
// }

// pub type DynamicAffixedBlocks = Affixed<DynamicBlock, DynamicBlock, DynamicBlock, DynamicBlock>;

// pub enum AffixerAllocateError<A> {
//     AllocatorError(A),
//     CouldNotSubdivide,
// }

// impl<A: Allocator<Layout, DynamicBlock>> Allocator<AffixedLayout, DynamicAffixedBlocks> for Affixer<A> {
//     type AllocateError = AffixerAllocateError<A::AllocateError>;

//     fn allocate(&self, layout: AffixedLayout) -> Result<DynamicAffixedBlocks, Self::AllocateError> {
//         let AffixedLayout {
//             layout,
//             middle_offset,
//             suffix_offset,
//         } = layout;
//         match self.allocator.allocate(layout) {
//             Err(e) => Err(AffixerAllocateError::AllocatorError(e)),
//             Ok(block) => block
//                 .try_subdivide_twice(middle_offset, suffix_offset)
//                 .ok_or(AffixerAllocateError::CouldNotSubdivide)
//                 .map(|(prefix, middle, suffix)| Affixed::new(prefix, middle, suffix, block)),
//         }
//     }
// }

// pub type BlueprintedDynamicAffixedBlocks = Affixed<DynamicBlock, DynamicBlock, DynamicBlock, Blueprinted<DynamicBlock>>;

// impl<A: Allocator<Layout, Blueprinted<DynamicBlock>>> Allocator<AffixedLayout, BlueprintedDynamicAffixedBlocks> for Affixer<A> {
//     type AllocateError = AffixerAllocateError<A::AllocateError>;

//     fn allocate(&self, layout: AffixedLayout) -> Result<BlueprintedDynamicAffixedBlocks, Self::AllocateError> {
//         todo!()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use crate::allocation::allocators::{system::Sys, deallocator::Deallocator, allocator::Allocator};

//     use super::*;

//     // #[test]
//     // fn affixer1() {
//     //     let affixed_layout = AffixedLayout::try_new(
//     //         Layout::from_size_align(4, 4).unwrap(),
//     //         Layout::from_size_align(512, 1024).unwrap(),
//     //         Layout::from_size_align(4, 4).unwrap()
//     //     ).unwrap();
//     //     let affixer = Affixer::new(Sys);
//     //     let affixed = affixer.allocate(affixed_layout);
//     // }
// }