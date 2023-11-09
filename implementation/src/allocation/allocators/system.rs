use std::{
    alloc::{GlobalAlloc, System},
    ptr::NonNull,
};

use crate::allocation::{
    blk::Block,
    traits::{Alloc, Dealloc, ZeroingAlloc},
};

pub struct SystemAlloc;

pub enum SystemAllocError {
    ZeroSizedLayout,
    SystemAllocFailed,
}

#[global_allocator]
static SYSTEM: System = System;

impl Alloc for SystemAlloc {
    type AllocError = SystemAllocError;

    fn alloc(&self, layout: std::alloc::Layout) -> Result<Block, Self::AllocError> {
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

// pub struct BoxAllocator {
//     data: Option<Box<[u8]>>,
// }

// impl BoxAllocator {
//     pub fn new() -> Self {
//         Self { data: None }
//     }
// }

// pub struct BoxBlock;

// pub enum BoxAllocError {
//     BoxAlreadyAllocated,
// }

// impl Alloc for BoxAllocator {
//     type Block = BoxBlock;

//     type AllocError = BoxAllocError;

//     fn alloc(&mut self, layout: std::alloc::Layout) -> Result<Self::Block, Self::AllocError> {
//         if let None = self.data {
//             self.data = Some(Vec::with_capacity(layout.size()).into_boxed_slice());
//             Ok(BoxBlock)
//         } else {
//             Err(BoxAllocError::BoxAlreadyAllocated)
//         }
//     }
// }

// pub enum BoxGetMutError {
//     BoxNotAllocated,
// }

// impl GetMut for BoxAllocator {
//     type Block = BoxBlock;

//     type GetMutError = BoxGetMutError;

//     fn get_mut(&mut self, block: Self::Block) -> Result<&mut [u8], Self::GetMutError> {
//         if let Some(b) = &mut self.data {
//             Ok(b)
//         } else {
//             Err(BoxGetMutError::BoxNotAllocated)
//         }
//     }
// }

// pub enum BoxDeallocError {
//     BoxNotAllocated
// }

// impl Dealloc for BoxAllocator {
//     type Block = BoxBlock;

//     type DeallocError = BoxDeallocError;

//     fn dealloc(&mut self, _: Self::Block) -> Option<Self::DeallocError> {
//         if self.data.is_some() {
//             self.data = None;
//             None
//         } else {
//             Some(BoxDeallocError::BoxNotAllocated)
//         }
//     }
// }
