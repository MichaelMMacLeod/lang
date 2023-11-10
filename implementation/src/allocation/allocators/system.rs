use std::{
    alloc::{GlobalAlloc, System},
    ptr::NonNull,
};

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
mod tests {
    use std::alloc::Layout;

    use crate::allocation::traits::alloc_affixed;

    use super::*;

    #[test]
    fn alloc1() {
        let s = SystemAlloc;
        const SIZE: usize = 1024;
        let l = Layout::from_size_align(SIZE, 1).unwrap();
        let b = s.alloc_zeroed(l).unwrap();
        assert_eq!(b.blueprint().expect("no blueprint"), l);
        assert!(Into::<NonNull<[u8]>>::into(b).len() >= SIZE);
        assert_eq!(Into::<*mut u8>::into(b).align_offset(1), 0);
        assert_eq!(unsafe { b.as_ref() }, &[0; SIZE]);
    }

    #[test]
    fn alloc2() {
        let prefix = Layout::from_size_align(4, 4).unwrap();
        let middle = Layout::from_size_align(512, 1024).unwrap();
        let suffix = Layout::from_size_align(4, 4).unwrap();
        let alloc_zeroed = |layout| SystemAlloc.alloc_zeroed(layout);
        let &[b_prefix, b_middle, b_suffix] = alloc_affixed(alloc_zeroed, prefix, middle, suffix)
            .unwrap()
            .blocks();
        assert!(b_prefix.blueprint().is_none());
        assert!(b_middle.blueprint().is_none());
        assert!(b_suffix.blueprint().is_none());
        assert_eq!(Into::<NonNull<[u8]>>::into(b_prefix).len(), 1024);
        assert_eq!(Into::<NonNull<[u8]>>::into(b_middle).len(), 512);
        assert!(Into::<NonNull<[u8]>>::into(b_suffix).len() >= 4);
        assert_eq!(Into::<*mut u8>::into(b_prefix).align_offset(4), 0);
        assert_eq!(Into::<*mut u8>::into(b_middle).align_offset(1024), 0);
        assert_eq!(Into::<*mut u8>::into(b_suffix).align_offset(4), 0);
        // const SIZE: usize = 1024;
        // let l = Layout::from_size_align(SIZE, 1).unwrap();
        // let b = s.alloc_zeroed(l).unwrap();
        // assert_eq!(b.blueprint().expect("no blueprint"), l);
        // assert!(Into::<NonNull<[u8]>>::into(b).len() >= SIZE);
        // assert_eq!(Into::<*mut u8>::into(b).align_offset(1), 0);
        // assert_eq!(
        //     unsafe { b.as_ref() },
        //     &[0; SIZE]
        // );
    }
}
