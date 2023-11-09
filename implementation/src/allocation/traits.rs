use std::{alloc::{Layout, LayoutError}, ptr::NonNull};

use super::blk::Block;

pub trait GoodSize {
    fn good_size_for(&self, layout: Layout) -> usize;
}

pub trait Alloc {
    type AllocError;

    fn alloc(&self, layout: Layout) -> Result<Block, Self::AllocError>;
}

pub trait ZeroingAlloc: Alloc {
    fn alloc_zeroed(&self, layout: Layout) -> Result<Block, Self::AllocError> {
        let blk = self.alloc(layout)?;
        let ptr: *mut u8 = blk.into();
        unsafe {
            ptr.write_bytes(0, NonNull::<[u8]>::from(blk).len());
        }
        Ok(blk)
    }
}

pub enum AffixAllocError<E> {
    LayoutError(LayoutError),
    AllocError(E),
}

impl<E> From<LayoutError> for AffixAllocError<E> {
    fn from(value: LayoutError) -> Self {
        Self::LayoutError(value)
    }
}

// pub trait PrefixAlloc: Alloc {
//     fn alloc_prefixed(&self, prefix: Layout, layout: Layout) -> Result<[Blk; 2], 
// }

// pub trait AffixAlloc: Alloc {
//     fn alloc_affixed(
//         &self,
//         prefix: Layout,
//         middle: Layout,
//         suffix: Layout,
//     ) -> Result<(Option<Blk>, Blk, Option<Blk>), AffixAllocError<Self::AllocError>> {
//         let (combined_layout, middle_offset) = prefix.extend(middle)?;
//         let (combined_layout, suffix_offset) = combined_layout.extend(suffix)?;
//         match self.alloc(combined_layout) {
//             Err(e) => Err(AffixAllocError::AllocError(e)),
//             Ok(blk) => {

//             },
//         }
//     }
// }

pub trait Dealloc {
    type DeallocError;

    unsafe fn dealloc(&self, block: Block) -> Option<Self::DeallocError>;
}

pub trait Owns {
    fn owns(&self, block: Block) -> bool;
}
