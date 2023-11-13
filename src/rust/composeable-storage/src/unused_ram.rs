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

/// Represents all of the unused [`Ram`] (random access memory) that
/// can be allocated with [`GlobalAlloc`].
///
/// In principle, "unused [`Ram`]" should mean [`Ram`] that can be
/// used by programs other than this one. In reality, implementations
/// of [`GlobalAlloc`] often won't---at least immediately---give
/// memory back to the operating system when [`std::alloc::dealloc`]
/// is called, reserving such freed memory for subsequent allocations
/// by the same program. The perceived bennefit of this behavior is
/// that when the program calls [`std::alloc::alloc`] later on, it is
/// possible to reuse the deallocated memory instead of asking the
/// operating system for new memory (e.g., via `brk()` or `mmap()`)
/// which can be very slow. Unfortunately, this makes the task of
/// implementing [`std::alloc::alloc`] a great challenge, and imposes
/// the overhead of keeping track of such freed memory on all
/// programs, each of which have access to context sensitive
/// information about how to partition the unused [`Ram`] which is
/// inaccesible to any [`GlobalAlloc`] implementation. The behavior of
/// [`std::alloc::alloc`] reusing deallocated [`Ram`] for efficiency
/// should not be relied upon; it should be expected that
/// [`UnusedRam::<G>::try_partition`] is very slow. This means it is
/// better to partition out a large slice of the unused [`Ram`] a
/// small number of times (and then further partition that [`Ram`] if
/// necessary) than it is to partition out small slices of unused
/// [`Ram`] a large number of times.
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

pub enum UnusedRamPartitionErrror {
    ZeroSizedLayout,
    GlobalAllocFailed,
}

/// Partitions the computer's unused random acces memory into a slice
/// of [`Ram`] and the rest of the [`UnusedRam`].
impl<G: GlobalAlloc> TryPartition<Ram, UnusedRam<G>, UnusedRamPartitionErrror> for UnusedRam<G> {
    fn try_partition(self) -> Result<Partitioned<Ram, UnusedRam<G>>, UnusedRamPartitionErrror> {
        if self.layout.size() == 0 {
            Err(UnusedRamPartitionErrror::ZeroSizedLayout)
        } else {
            let ptr = unsafe {
                // Safety: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html#safety-1
                // Since we know layout is not zero-sized, this should be safe.
                std::alloc::GlobalAlloc::alloc(&self.global_alloc, self.layout)
            };

            NonNull::new(ptr)
                .ok_or(UnusedRamPartitionErrror::GlobalAllocFailed)
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
/// memory back into the [`UnusedRam`] so it may be used by other
/// programs.
///
/// Safety: the [`Ram`] must have been originally partitioned from the
/// same [`UnusedRam`].
impl<G: GlobalAlloc> TryMergeUnsafe<Ram, UnusedRam<G>, UnusedRam<G>, ()> for UnusedRam<G> {
    unsafe fn try_merge(p: Partitioned<Ram, UnusedRam<G>>) -> Result<UnusedRam<G>, ()> {
        p.transform(|slice, ram| {
            std::alloc::dealloc(slice.start_ptr(), ram.layout);
            Ok(ram)
        })
    }
}
