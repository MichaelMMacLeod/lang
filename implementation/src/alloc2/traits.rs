use std::{alloc::Layout, ptr::NonNull};

use super::blk::Blk;

pub trait GoodSize {
    fn good_size_for(&self, layout: Layout) -> usize;
}

pub trait Alloc {
    type AllocError;

    fn alloc(&self, layout: Layout) -> Result<Blk, Self::AllocError>;
}

pub trait Dealloc {
    type DeallocError;

    unsafe fn dealloc(&self, block: Blk) -> Option<Self::DeallocError>;
}

pub trait Owns {
    fn owns(&self, block: Blk) -> bool;
}

// pub trait ReferenceCounting {
//     fn increment_reference_count(&self, ptr: Ptr)
// }

// pub trait Alloc {
//     fn alloc(&self, layout: Layout) -> Ptr;
// }

// impl<A: Alloc> TryAlloc for A {
//     type AllocError = ();

//     fn try_alloc(&self, layout: Layout) -> Result<Ptr, Self::AllocError> {
//         Ok(self.alloc(layout))
//     }
// }

// pub trait GoodAllocSize {
//     fn good_alloc_size(layout: Layout) -> usize;
// }
