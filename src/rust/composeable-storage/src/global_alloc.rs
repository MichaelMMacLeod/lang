use std::{
    alloc::{Layout, System},
    ptr::NonNull,
};

use crate::{
    alignment::Alignment,
    global_alloc_slice::GlobalAllocSlice,
    partition::{Partition, Partitioned, TryPartition},
    slice::Slice,
    merge::TryMergeUnsafe,
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

impl TryPartition<GlobalAllocSlice, GlobalAlloc, GlobalAllocPartitionError> for GlobalAlloc {
    fn try_partition(
        self,
    ) -> Result<Partitioned<GlobalAllocSlice, GlobalAlloc>, GlobalAllocPartitionError> {
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
                    let slice = Slice::new(slice_ptr);
                    let global_alloc_slice = GlobalAllocSlice::new(slice, self.layout);
                    Partitioned::new(global_alloc_slice, self)
                })
        }
    }
}

// impl TryMergeUnsafe<GlobalAllocSlice, GlobalAlloc, 