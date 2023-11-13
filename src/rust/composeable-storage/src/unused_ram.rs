use std::{
    alloc::{GlobalAlloc, Layout},
    convert::Infallible,
    ptr::NonNull,
};

use crate::{
    alignment::Alignment,
    merge::TryMergeUnsafe,
    partition::{Partitioned, TryPartition},
    ram::Ram,
};

/// Represents all of the unused slices of RAM which can be returned
/// from [`GlobalAlloc::alloc`] using the same [`Layout`]. See [`Ram`]
/// for an example of allocation and deallocation.
///
/// In principle, "unused RAM" should mean RAM that can be used by
/// other processes. In reality, implementations of
/// [`std::alloc::GlobalAlloc`] often won't---at least
/// immediately---give memory back to the operating system when
/// [`GlobalAlloc::dealloc`] is called, reserving such freed memory
/// for subsequent allocations. The perceived benefit of this behavior
/// is that if [`GlobalAlloc::alloc`] is called later on, it may be
/// possible to avoid asking the operating system for more RAM via
/// `brk()` or `mmap()` (on POSIX-compliant operating systems) which
/// can be very slow. Unfortunately, this imposes the overhead of
/// keeping track of such freed memory on all processes with a global
/// allocator. Given that each process almost certainly has more
/// information about the way it partitions the unused RAM than the
/// global allocator, it makes sense to parameterize this type on a
/// simple global allocator which doesn't hold on to any deallocated
/// ram. The behavior of [`GlobalAlloc::alloc`] reusing deallocated
/// RAM for efficiency should not be relied upon; it should be
/// expected that [`UnusedRam::<G>::try_partition`] is very slow. This
/// means it is better to partition out a large slice of the unused
/// RAM a small number of times (and then further partition that
/// [`Ram`] if necessary) than it is to partition out small slices of
/// unused RAM a large number of times.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum UnusedRamPartitionErrror {
    ZeroSizedLayout,
    GlobalAllocFailed,
}

/// Partitions the computer's unused RAM into a slice of [`Ram`] and
/// the rest of the [`UnusedRam`].
impl<G: GlobalAlloc> TryPartition<Ram> for UnusedRam<G> {
    type TryPartitionError = UnusedRamPartitionErrror;

    fn try_partition(self) -> Result<Partitioned<Ram, Self>, UnusedRamPartitionErrror> {
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

/// Merges a no-longer-needed slice of the computer's RAM back into
/// its unused RAM so it may be used by other processes.
///
/// Safety: the [`Ram`] must have been originally partitioned from the
/// same [`UnusedRam`] and must not have already been merged.
impl<G: GlobalAlloc> TryMergeUnsafe<Ram> for UnusedRam<G> {
    type TryMergeUnsafeError = Infallible;

    unsafe fn try_merge_unsafe(self, ram: Ram) -> Result<Self, Self::TryMergeUnsafeError> {
        std::alloc::GlobalAlloc::dealloc(&self.global_alloc, ram.start_ptr(), self.layout);
        Ok(self)
    }
}
