use std::{
    alloc::{Layout, System},
    ptr::NonNull,
};

use crate::{
    alignment::Alignment,
    merge::TryMergeUnsafe,
    partition::{Partitioned, TryPartition},
    slice::Slice,
};

#[global_allocator]
static GLOBAL_ALLOC: System = System;

pub struct Ram {
    layout: Layout,
}

impl Ram {
    pub fn new(layout: Layout) -> Self {
        Self { layout }
    }
}

pub enum RamPartitionErrror {
    ZeroSizedLayout,
    GlobalAllocFailed,
}

impl TryPartition<Slice, Ram, RamPartitionErrror> for Ram {
    fn try_partition(self) -> Result<Partitioned<Slice, Ram>, RamPartitionErrror> {
        if self.layout.size() == 0 {
            Err(RamPartitionErrror::ZeroSizedLayout)
        } else {
            let ptr = unsafe {
                // Safety: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html#safety-1
                // Since we know layout is not zero-sized, this should be safe.
                std::alloc::GlobalAlloc::alloc(&GLOBAL_ALLOC, self.layout)
            };

            NonNull::new(ptr)
                .ok_or(RamPartitionErrror::GlobalAllocFailed)
                .map(|ptr| {
                    let slice_ptr = NonNull::slice_from_raw_parts(ptr, self.layout.size());
                    let alignment = unsafe { Alignment::new_unchecked(self.layout.align()) };
                    let slice = Slice::new(slice_ptr, alignment);
                    Partitioned::new(slice, self)
                })
        }
    }
}

impl TryMergeUnsafe<Slice, Ram, Ram, ()> for Ram {
    unsafe fn try_merge(p: Partitioned<Slice, Ram>) -> Result<Ram, ()> {
        p.transform(|slice, ram| {
            std::alloc::dealloc(slice.start_ptr(), ram.layout);
            Ok(ram)
        })
    }
}
