use std::{
    alloc::{GlobalAlloc, Layout},
    ptr::NonNull,
};

use crate::{
    alignment::Alignment,
    merge::TryMergeUnsafe,
    partition::{Partitioned, TryPartition},
    slice::Ram,
};

/// Represents all of the unused RAM (random access memory) that can
/// be allocated with [`GlobalAlloc`].
pub struct UnusedRam<G: GlobalAlloc> {
    global_alloc: G,
    layout: Layout,
}

impl<G: GlobalAlloc> UnusedRam<G> {
    pub fn new(global_alloc: G, layout: Layout) -> Self {
        Self {
            global_alloc,
            layout,
        }
    }
}

pub enum RamPartitionErrror {
    ZeroSizedLayout,
    GlobalAllocFailed,
}

/// Partitions the computer's unused ramdom acces memory into a slice
/// of [`Ram`] and the rest of the [`UnusedRam`].
impl<G: GlobalAlloc> TryPartition<Ram, UnusedRam<G>, RamPartitionErrror> for UnusedRam<G> {
    fn try_partition(self) -> Result<Partitioned<Ram, UnusedRam<G>>, RamPartitionErrror> {
        if self.layout.size() == 0 {
            Err(RamPartitionErrror::ZeroSizedLayout)
        } else {
            let ptr = unsafe {
                // Safety: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html#safety-1
                // Since we know layout is not zero-sized, this should be safe.
                std::alloc::GlobalAlloc::alloc(&self.global_alloc, self.layout)
            };

            NonNull::new(ptr)
                .ok_or(RamPartitionErrror::GlobalAllocFailed)
                .map(|ptr| {
                    let slice_ptr = NonNull::slice_from_raw_parts(ptr, self.layout.size());
                    // Safety: layout.align() returns a positive integer.
                    let alignment = unsafe { Alignment::new_unchecked(self.layout.align()) };
                    let slice = Ram::new(slice_ptr, alignment);
                    Partitioned::new(slice, self)
                })
        }
    }
}

/// Merges a no-longer-needed slice of the computer's random access
/// memory back into the [`UnusedRam`].
///
/// Safety: the [`Ram`] must have been originally partitioned from the
/// same slice of [`UnusedRam`].
impl<G: GlobalAlloc> TryMergeUnsafe<Ram, UnusedRam<G>, UnusedRam<G>, ()> for UnusedRam<G> {
    unsafe fn try_merge(p: Partitioned<Ram, UnusedRam<G>>) -> Result<UnusedRam<G>, ()> {
        p.transform(|slice, ram| {
            std::alloc::dealloc(slice.start_ptr(), ram.layout);
            Ok(ram)
        })
    }
}
