use std::{ops::Deref, ptr::NonNull, u8};

use crate::alignment::Alignment;

use crate::all;

all! {
    #[cfg(doc)]
    use crate::unused_ram::UnusedRam;
    use crate::partition::TryPartition;
    use crate::merge::MergeUnsafe;
}

/// An aligned slice of the computer's random access memory. This can
/// be created via [`UnusedRam::<G>::try_partition`]. All RAM should
/// eventually be merged back into the [`UnusedRam`] via
/// [`UnusedRam::<G>::merge_unsafe`].
///
/// # Examples
///
/// ```
/// use std::alloc::{Layout, System};
/// use composeable_storage::{
///     unused_ram::UnusedRam, 
///     partition::TryPartition,
///     merge::TryMergeUnsafe,
/// };
///
/// // Partition a slice of ram out of the unused RAM.
/// let layout = Layout::from_size_align(16, 64).unwrap();
/// let unused_ram = UnusedRam::new(System, layout);
/// let (ram, unused_ram) = unused_ram.try_partition().unwrap().into();
///
/// // Initialize every byte of the RAM with 42.
/// unsafe { ram.start_ptr().write_bytes(42, ram.len()) };
/// for &byte in unsafe { ram.as_ref() } {
///     assert_eq!(byte, 42);
/// }
///
/// // Merge the slice of RAM back into the unused RAM.
/// // Don't forget to do this; it's a memory leak if
/// // you dont!
/// unsafe { unused_ram.merge_unsafe(ram) };
/// ```
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Ram {
    slice: NonNull<[u8]>,
    alignment: Alignment,
}

impl Ram {
    pub fn new(slice: NonNull<[u8]>, alignment: Alignment) -> Self {
        Self { slice, alignment }
    }

    pub fn alignment(&self) -> Alignment {
        self.alignment
    }

    pub fn slice_ptr(&self) -> *mut [u8] {
        self.slice.as_ptr()
    }

    pub fn start_ptr(&self) -> *mut u8 {
        self.as_ptr() as *mut u8
    }
}

impl Deref for Ram {
    type Target = NonNull<[u8]>;

    fn deref(&self) -> &Self::Target {
        &self.slice
    }
}
