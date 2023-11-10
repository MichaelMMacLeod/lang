use std::{
    alloc::{Layout, System},
    ptr::NonNull,
};

use crate::allocation::blocks::{blueprinted::Blueprinted, block_dynamic::DynamicBlock};

use super::allocator::Allocator;

#[global_allocator]
static SYSTEM: System = System;

struct Sys;

#[derive(Debug)]
pub enum SystemAllocateError {
    ZeroSizedLayout,
    SystemAllocateFailed,
}

impl Allocator<Blueprinted<DynamicBlock>> for Sys {
    type AllocateError = SystemAllocateError;

    fn allocate(
        &self,
        layout: Layout,
    ) -> Result<Blueprinted<DynamicBlock>, Self::AllocateError> {
        if layout.size() == 0 {
            Err(SystemAllocateError::ZeroSizedLayout)
        } else {
            let r = unsafe {
                // safety: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html#safety-1
                std::alloc::GlobalAlloc::alloc(&SYSTEM, layout)
            };

            NonNull::new(r)
                .ok_or(SystemAllocateError::SystemAllocateFailed)
                .map(|n| {
                    Blueprinted::new(
                        DynamicBlock::new(NonNull::slice_from_raw_parts(n, layout.size())),
                        layout,
                    )
                })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::allocation::blocks::map_dynamic_block;

    use super::*;

    #[test]
    fn alloc1() {
        let size = 128;
        let align = 1024;
        let layout = Layout::from_size_align(size, align).unwrap();
        let block = Sys.allocate(layout).unwrap();
        assert_eq!(block.blueprint(), layout);
        block.map(|d| d);
    }
}

// use std::{
//     alloc::{GlobalAlloc, Layout, System},
//     ptr::NonNull,
// };

// use crate::allocation::blocks::{
//     blueprinted::Blueprinted, unsized_uninitialized::UnsizedUninitialized,
// };

// use super::traits::Allocator;

// // use crate::allocation::{
// //     block::Block,
// //     traits::{Alloc, Dealloc, ZeroingAlloc},
// // };

// pub struct Sys;

// impl Allocator<Blueprinted<UnsizedUninitialized>> for Sys {
//     type AllocateError = SystemAllocError;

//     fn allocate(
//         &self,
//         layout: Layout,
//     ) -> Result<Blueprinted<UnsizedUninitialized>, Self::AllocateError> {
//
//     }
// }

// // pub enum SystemDeallocError {
// //     NoBlueprint,
// // }

// // impl Sys {
// //

// //     unsafe fn dealloc(&self, block: Block) -> Option<SystemDeallocError> {
// //         if let Some(blueprint) = block.blueprint() {
// //             // safety: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html#safety-2
// //             System.dealloc(block.into(), blueprint);
// //             None
// //         } else {
// //             Some(SystemDeallocError::NoBlueprint)
// //         }
// //     }
// // }

// // // #[cfg(test)]
// // // mod tests {
// // //     use std::{alloc::Layout, io::Write};

// // //     use crate::allocation::traits::{alloc_affixed, dealloc_affixed};

// // //     use super::*;

// // //     use proptest::proptest;

// // //     #[test]
// // //     fn alloc1() {
// // //         let s = Sys;
// // //         const SIZE: usize = 1024;
// // //         let l = Layout::from_size_align(SIZE, 1).unwrap();
// // //         let b = s.alloc_zeroed(l).unwrap();
// // //         assert_eq!(b.blueprint().expect("no blueprint"), l);
// // //         assert!(b.len() >= SIZE);
// // //         assert_eq!(Into::<*const u8>::into(b).align_offset(1), 0);
// // //         assert_eq!(unsafe { b.as_ref() }, &[0; SIZE]);
// // //         unsafe { s.dealloc(b) };
// // //     }

// // //     #[test]
// // //     fn alloc2() {
// // //         let s = Sys;
// // //         let prefix = Layout::from_size_align(4, 4).unwrap();
// // //         let middle = Layout::from_size_align(512, 1024).unwrap();
// // //         let suffix = Layout::from_size_align(4, 4).unwrap();
// // //         let affixed = alloc_affixed(|l| s.alloc_zeroed(l), prefix, middle, suffix).unwrap();
// // //         {
// // //             let &[b_prefix, b_middle, b_suffix] = affixed.blocks();
// // //             assert!(b_prefix.blueprint().is_none());
// // //             assert!(b_middle.blueprint().is_none());
// // //             assert!(b_suffix.blueprint().is_none());
// // //             assert_eq!(b_prefix.len(), 1024);
// // //             assert_eq!(b_middle.len(), 512);
// // //             assert!(b_suffix.len() >= 4);
// // //             assert_eq!(Into::<*const u8>::into(b_prefix).align_offset(4), 0);
// // //             assert_eq!(Into::<*const u8>::into(b_middle).align_offset(1024), 0);
// // //             assert_eq!(Into::<*const u8>::into(b_suffix).align_offset(4), 0);
// // //             unsafe {
// // //                 Into::<*mut u8>::into(b_prefix).write_bytes(1, b_prefix.len());
// // //                 Into::<*mut u8>::into(b_middle).write_bytes(2, b_middle.len());
// // //                 Into::<*mut u8>::into(b_suffix).write_bytes(3, b_suffix.len());
// // //             }
// // //             // These should all be non-overlapping. MIRI might complain about it here if they
// // //             // aren't.
// // //             let prefix_ptr = unsafe { b_prefix.as_mut() };
// // //             let middle_ptr = unsafe { b_middle.as_mut() };
// // //             let suffix_ptr = unsafe { b_suffix.as_mut() };
// // //             for v in prefix_ptr.iter_mut() {
// // //                 assert_eq!(*v, 1);
// // //                 *v = 11;
// // //                 assert_eq!(*v, 11);
// // //             }
// // //             for v in middle_ptr {
// // //                 assert_eq!(*v, 2);
// // //                 *v = 22;
// // //                 assert_eq!(*v, 22);
// // //             }
// // //             for v in suffix_ptr {
// // //                 assert_eq!(*v, 3);
// // //                 *v = 33;
// // //                 assert_eq!(*v, 33);
// // //             }
// // //         }
// // //         assert!(unsafe { dealloc_affixed(|b| s.dealloc(b), &affixed).is_none() });
// // //     }

// // //     proptest! {
// // //         #[test]
// // //         fn prop_alloc1(
// // //             ps in 0..64usize, pa in 1..8u32,
// // //             ms in 1..64usize, ma in 1..8u32,
// // //             ss in 0..64usize, sa in 1..8u32,
// // //         ) {
// // //             use std::alloc::Layout;
// // //             use crate::allocation::traits::*;
// // //             let s = Sys;
// // //             let pl = Layout::from_size_align(ps, 2usize.pow(pa)).unwrap();
// // //             let ml = Layout::from_size_align(ms, 2usize.pow(ma)).unwrap();
// // //             let sl = Layout::from_size_align(ss, 2usize.pow(sa)).unwrap();
// // //             let affixed = alloc_affixed(|l| s.alloc_zeroed(l), pl, ml, sl).unwrap();
// // //             {
// // //                 let &[b_prefix, b_middle, b_suffix] = affixed.blocks();
// // //                 assert!(b_prefix.blueprint().is_none());
// // //                 assert!(b_middle.blueprint().is_none());
// // //                 assert!(b_suffix.blueprint().is_none());
// // //                 assert!(b_prefix.len() >= ps);
// // //                 assert!(b_middle.len() >= ms);
// // //                 assert!(b_suffix.len() >= ss);
// // //             }
// // //             assert!(unsafe { dealloc_affixed(|b| s.dealloc(b), &affixed).is_none() });
// // //         }
// // //     }
// // // }
