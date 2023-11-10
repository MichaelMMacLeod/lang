use std::{
    alloc::{GlobalAlloc, System},
    ptr::NonNull,
};

#[cfg(test)]
use proptest::proptest;

use crate::allocation::{
    block::Block,
    traits::{Alloc, Dealloc, ZeroingAlloc},
};

pub struct SystemAlloc;

#[derive(Debug)]
pub enum SystemAllocError {
    ZeroSizedLayout,
    SystemAllocFailed,
}

#[global_allocator]
static SYSTEM: System = System;

impl Alloc for SystemAlloc {
    type AllocError = SystemAllocError;

    fn alloc_uninitialized(&self, layout: std::alloc::Layout) -> Result<Block, Self::AllocError> {
        if layout.size() == 0 {
            Err(SystemAllocError::ZeroSizedLayout)
        } else {
            let r = unsafe {
                // safety: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html#safety-1
                SYSTEM.alloc(layout)
            };

            NonNull::new(r)
                .ok_or(SystemAllocError::SystemAllocFailed)
                .map(|n| {
                    Block::new(
                        NonNull::slice_from_raw_parts(n, layout.size()),
                        Some(layout),
                    )
                })
        }
    }
}

pub enum SystemDeallocError {
    NoBlueprint,
}

impl Dealloc for SystemAlloc {
    type DeallocError = SystemDeallocError;

    unsafe fn dealloc(&self, block: Block) -> Option<Self::DeallocError> {
        if let Some(blueprint) = block.blueprint() {
            // safety: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html#safety-2
            System.dealloc(block.into(), blueprint);
            None
        } else {
            Some(SystemDeallocError::NoBlueprint)
        }
    }
}

#[cfg(test)]
proptest! {
    #[test]
    fn prop_alloc1(
        ps in 0..64usize, pa in 1..8u32,
        ms in 1..64usize, ma in 1..8u32,
        ss in 0..64usize, sa in 1..8u32,
    ) {
        use std::alloc::Layout;
        use crate::allocation::traits::*;
        let s = SystemAlloc;
        let pl = Layout::from_size_align(ps, 2usize.pow(pa)).unwrap();
        let ml = Layout::from_size_align(ms, 2usize.pow(ma)).unwrap();
        let sl = Layout::from_size_align(ss, 2usize.pow(sa)).unwrap();
        let affixed = alloc_affixed(|l| s.alloc_zeroed(l), pl, ml, sl).unwrap();
        {
            let &[b_prefix, b_middle, b_suffix] = affixed.blocks();
            assert!(b_prefix.blueprint().is_none());
            assert!(b_middle.blueprint().is_none());
            assert!(b_suffix.blueprint().is_none());
            assert!(b_prefix.len() >= ps);
            assert!(b_middle.len() >= ms);
            assert!(b_suffix.len() >= ss);
        }
        assert!(unsafe { dealloc_affixed(|b| s.dealloc(b), &affixed).is_none() });
    }
}

#[cfg(test)]
mod tests {
    use std::{alloc::Layout, io::Write};

    use crate::allocation::traits::{alloc_affixed, dealloc_affixed};

    use super::*;

    #[test]
    fn alloc1() {
        let s = SystemAlloc;
        const SIZE: usize = 1024;
        let l = Layout::from_size_align(SIZE, 1).unwrap();
        let b = s.alloc_zeroed(l).unwrap();
        assert_eq!(b.blueprint().expect("no blueprint"), l);
        assert!(b.len() >= SIZE);
        assert_eq!(Into::<*const u8>::into(b).align_offset(1), 0);
        assert_eq!(unsafe { b.as_ref() }, &[0; SIZE]);
        unsafe { s.dealloc(b) };
    }

    #[test]
    fn alloc2() {
        let s = SystemAlloc;
        let prefix = Layout::from_size_align(4, 4).unwrap();
        let middle = Layout::from_size_align(512, 1024).unwrap();
        let suffix = Layout::from_size_align(4, 4).unwrap();
        let affixed = alloc_affixed(|l| s.alloc_zeroed(l), prefix, middle, suffix).unwrap();
        {
            let &[b_prefix, b_middle, b_suffix] = affixed.blocks();
            assert!(b_prefix.blueprint().is_none());
            assert!(b_middle.blueprint().is_none());
            assert!(b_suffix.blueprint().is_none());
            assert_eq!(b_prefix.len(), 1024);
            assert_eq!(b_middle.len(), 512);
            assert!(b_suffix.len() >= 4);
            assert_eq!(Into::<*const u8>::into(b_prefix).align_offset(4), 0);
            assert_eq!(Into::<*const u8>::into(b_middle).align_offset(1024), 0);
            assert_eq!(Into::<*const u8>::into(b_suffix).align_offset(4), 0);
            unsafe {
                Into::<*mut u8>::into(b_prefix).write_bytes(1, b_prefix.len());
                Into::<*mut u8>::into(b_middle).write_bytes(2, b_middle.len());
                Into::<*mut u8>::into(b_suffix).write_bytes(3, b_suffix.len());
            }
            // These should all be non-overlapping. MIRI might complain about it here if they
            // aren't.
            let prefix_ptr = unsafe { b_prefix.as_mut() };
            let middle_ptr = unsafe { b_middle.as_mut() };
            let suffix_ptr = unsafe { b_suffix.as_mut() };
            for v in prefix_ptr.iter_mut() {
                assert_eq!(*v, 1);
                *v = 11;
                assert_eq!(*v, 11);
            }
            for v in middle_ptr {
                assert_eq!(*v, 2);
                *v = 22;
                assert_eq!(*v, 22);
            }
            for v in suffix_ptr {
                assert_eq!(*v, 3);
                *v = 33;
                assert_eq!(*v, 33);
            }
        }
        assert!(unsafe { dealloc_affixed(|b| s.dealloc(b), &affixed).is_none() });
    }
}
