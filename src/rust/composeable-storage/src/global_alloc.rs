use std::{
    alloc::{Layout, System},
    ptr::NonNull,
};

use crate::{
    alignment::Alignment,
    partition::{Partitioned, TryPartition},
    slice::AlignedSlice,
};

#[global_allocator]
static GLOBAL_ALLOC: System = System;

pub enum GlobalAllocSize {
    Bytes(),
}

pub struct GlobalAlloc {
    layout: Layout,
}

pub enum GlobalAllocPartitionError {
    ZeroSizedLayout,
    GlobalAllocFailed,
}

impl TryPartition<AlignedSlice, GlobalAlloc, GlobalAllocPartitionError> for GlobalAlloc {
    fn try_partition(
        self,
    ) -> Result<Partitioned<AlignedSlice, GlobalAlloc>, GlobalAllocPartitionError> {
        if self.layout.size() == 0 {
            Err(GlobalAllocPartitionError::ZeroSizedLayout)
        } else {
            let ptr = unsafe {
                // Safety: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html#safety-1
                // Since we know layout is not zero-sized, this should be safe.
                std::alloc::GlobalAlloc::alloc(&GLOBAL_ALLOC, self.layout)
            };

            NonNull::new(ptr)
                .ok_or(GlobalAllocPartitionError::GlobalAllocFailed)
                .map(|ptr| {
                    let slice_ptr = NonNull::slice_from_raw_parts(ptr, self.layout.size());
                    let alignment = unsafe { Alignment::new_unchecked(self.layout.align()) };
                    let slice = AlignedSlice::new(slice_ptr, alignment);
                    Partitioned::new(slice, self)
                })
        }
    }
}

// impl TryMergeUnsafe<GlobalAllocSlice, GlobalAlloc,
