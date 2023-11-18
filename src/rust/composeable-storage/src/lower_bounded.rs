use std::{
    alloc::{GlobalAlloc, Layout, LayoutError},
    ptr::NonNull,
};

use crate::{
    aligned::AlignedLength,
    aligned_slice::AlignedSlice,
    alignment::Alignment,
    arithmetic_errors::Overflow,
    merge::{MergeUnsafe, TryMerge, TryMergeTransform},
    partition::{Partitioned, TryPartition},
    units::Bytes,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LowerBounded<L, S> {
    alignment: Alignment,
    length: L,
    storage: S,
}

impl<L, S> LowerBounded<L, S> {
    pub fn new(alignment: Alignment, length: L, storage: S) -> Self {
        Self {
            alignment,
            length,
            storage,
        }
    }

    pub fn alignment(&self) -> Alignment {
        self.alignment
    }

    pub fn length_ref(&self) -> &L {
        &self.length
    }

    pub fn storage_ref(&self) -> &S {
        &self.storage
    }
}

impl<L, S> From<LowerBounded<L, S>> for (Alignment, L, S) {
    fn from(value: LowerBounded<L, S>) -> Self {
        (value.alignment, value.length, value.storage)
    }
}

impl<G: GlobalAlloc> LowerBounded<Bytes<usize>, G> {
    pub fn try_new_global(
        alignment: Alignment,
        bytes_per_partition: Bytes<usize>,
        global_alloc: G,
    ) -> Option<Self> {
        Layout::from_size_align(bytes_per_partition.count, alignment.as_usize())
            .ok()
            .map(|_| Self::new(alignment, bytes_per_partition, global_alloc))
    }

    pub fn layout(&self) -> Layout {
        // Safety: the only way to create Self is through try_new which checks these invariants.
        unsafe { Layout::from_size_align_unchecked(self.length.count, self.alignment.as_usize()) }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GlobalAllocFailed;

impl<G: GlobalAlloc> TryPartition<AlignedSlice> for LowerBounded<Bytes<usize>, G> {
    type TryPartitionError = GlobalAllocFailed;

    fn try_partition(self) -> Result<Partitioned<AlignedSlice, Self>, Self::TryPartitionError> {
        let ptr = unsafe {
            // Safety: Layout size can't be zero so this is safe.
            std::alloc::GlobalAlloc::alloc(self.storage_ref(), self.layout())
        };

        NonNull::new(ptr).ok_or(GlobalAllocFailed).map(|ptr| {
            let slice_ptr = NonNull::slice_from_raw_parts(ptr, self.layout().size());
            let slice = AlignedSlice::new(slice_ptr, self.alignment());
            Partitioned::new(slice, self)
        })
    }
}

impl<G: GlobalAlloc> MergeUnsafe<AlignedSlice> for LowerBounded<Bytes<usize>, G> {
    unsafe fn merge_unsafe(self, ram: AlignedSlice) -> Self {
        std::alloc::GlobalAlloc::dealloc(self.storage_ref(), ram.start_ptr(), self.layout());
        self
    }
}

impl<G: GlobalAlloc> TryMergeTransform<AlignedLength<Bytes<usize>>>
    for G
{
    type TryMergeTransformError = Overflow;
    type New = LowerBounded<Bytes<usize>, G>;

    fn try_merge_transform(
        self,
        data: AlignedLength<Bytes<usize>>,
    ) -> Result<LowerBounded<Bytes<usize>, G>, Self::TryMergeTransformError> {
        let size_bytes = data.unaligned_length();
        let align = data.alignment().as_usize();
        Layout::from_size_align(size_bytes.count, align)
            .map_err(|_| Overflow)
            .map(|_| LowerBounded {
                alignment: data.alignment(),
                length: size_bytes,
                storage: self,
            })
    }
}

#[cfg(test)]
mod test {
    use core::slice;
    use std::alloc::System;

    use super::*;

    #[test]
    fn global1() {
        let alignment = Alignment::new(64).unwrap();
        let bytes = Bytes { count: 128 };
        let g = System
            .try_merge_transform(AlignedLength::new(alignment, bytes))
            .unwrap();
        let (slice, g) = g.try_partition().unwrap().into();
        unsafe { slice.start_ptr().write_bytes(42, bytes.count) };
        for &byte in unsafe { slice.as_ref() } {
            assert_eq!(byte, 42);
        }
        unsafe { g.merge_unsafe(slice) };
    }
}
