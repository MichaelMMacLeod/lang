use std::{
    alloc::{GlobalAlloc, Layout},
    num::NonZeroUsize,
    ptr::NonNull,
};

use crate::{
    alignment::Alignment,
    // bytes::Bytes,
    merge::MergeUnsafe,
    partition::{Partitioned, TryPartition},
    aligned_slice::AlignedSlice,
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
/// which can be very slow. Unfortunately, this imposes the overhead
/// of keeping track of such freed memory on all processes with a
/// global allocator. Given that each process almost certainly has
/// more information about the way it partitions the unused RAM than
/// the global allocator, it makes sense to parameterize this type on
/// a simple global allocator which doesn't hold on to any deallocated
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

    // invariant: layout.size() > 0
    layout: Layout,
}

impl<G: GlobalAlloc> UnusedRam<G> {
    /// Returns the unused slices of ram managed by a specific
    /// implementation of [`GlobalAlloc`] which are no less than the
    /// given size in length and which are all aligned to
    /// `alignment`. Returns [`None`] if `size` becomes larger than
    /// [`isize::MAX`] when rounded up to `alignment`.
    pub fn try_new(
        global_alloc: G,
        size: NonZeroUsize,
        alignment: Alignment,
    ) -> Option<Self> {
        Layout::from_size_align(size.into(), alignment.into())
            .ok()
            .map(|layout| Self {
                global_alloc,
                layout,
            })
    }

    // pub fn try_new_unitified(
    //     global_alloc: G,
    //     size: bytes,
    //     alignment: Alignment,
    // ) -> Option<Self> {
    //     todo!()
    // }
}

#[derive(Clone, Copy, Debug)]
pub struct GlobalAllocFailed;

/// Partitions the computer's unused RAM into a slice of [`Ram`] and
/// the rest of the [`UnusedRam`].
impl<G: GlobalAlloc> TryPartition<AlignedSlice> for UnusedRam<G> {
    type TryPartitionError = GlobalAllocFailed;

    fn try_partition(self) -> Result<Partitioned<AlignedSlice, Self>, Self::TryPartitionError> {
        let ptr = unsafe {
            // Safety: https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html#safety-1
            // Since we know layout is not zero-sized (from the invariant on our struct),
            // this should be safe.
            std::alloc::GlobalAlloc::alloc(&self.global_alloc, self.layout)
        };

        NonNull::new(ptr).ok_or(GlobalAllocFailed).map(|ptr| {
            let slice_ptr = NonNull::slice_from_raw_parts(ptr, self.layout.size());

            // Safety: layout.align() returns a positive integer.
            let alignment = unsafe { Alignment::new_unchecked(self.layout.align()) };

            let slice = AlignedSlice::new(slice_ptr, alignment);
            Partitioned::new(slice, self)
        })
    }
}

/// Merges a no-longer-needed slice of the computer's RAM back into
/// its unused RAM so it may be used by other processes.
///
/// Safety: the [`Ram`] must have been originally partitioned from the
/// same [`UnusedRam`] and must not have already been merged.
impl<G: GlobalAlloc> MergeUnsafe<AlignedSlice> for UnusedRam<G> {
    unsafe fn merge_unsafe(self, ram: AlignedSlice) -> Self {
        std::alloc::GlobalAlloc::dealloc(&self.global_alloc, ram.start_ptr(), self.layout);
        self
    }
}
