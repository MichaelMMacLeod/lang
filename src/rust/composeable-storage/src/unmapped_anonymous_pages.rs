use std::{ffi::c_void, io::Error, num::NonZeroUsize};

use rustix::{
    io::Errno,
    mm::{mmap_anonymous, munmap, MapFlags, ProtFlags},
};

use crate::{
    anonymous_pages::AnonymousPages,
    merge::TryMergeUnsafe,
    partition::{Partitioned, TryPartition},
    units::information::{page_size_bytes, Bytes, Pages},
};

pub struct UnmappedAnonymousPages {
    // Invariant: a value of this struct may not exist if the system
    // page size (page_size_bytes()) is not a power of two.
    len_bytes: Bytes<NonZeroUsize>,
}

impl UnmappedAnonymousPages {
    #[cfg(unix)]
    pub fn try_new_normal(pages_per_partition: Pages<NonZeroUsize>) -> Option<Self> {
        if !usize::from(page_size_bytes().count).is_power_of_two() {
            // This probably will never happen but I couldn't find it
            // stated in POSIX, so we're checking for it here.
            None
        } else {
            pages_per_partition
                .try_into()
                .ok()
                .map(|len_bytes| Self { len_bytes })
        }
    }
}

impl TryPartition<AnonymousPages> for UnmappedAnonymousPages {
    type TryPartitionError = Errno;

    fn try_partition(self) -> Result<Partitioned<AnonymousPages, Self>, Self::TryPartitionError> {
        let start_ptr: *mut c_void = unsafe {
            mmap_anonymous(
                std::ptr::null_mut(),
                usize::from(self.len_bytes.count),
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::PRIVATE,
            )
        }?;
        Ok(Partitioned::new(
            AnonymousPages::new(start_ptr, self.len_bytes),
            self,
        ))
    }
}

impl TryMergeUnsafe<AnonymousPages> for UnmappedAnonymousPages {
    type TryMergeUnsafeError = Error;

    unsafe fn try_merge_unsafe(
        self,
        data: AnonymousPages,
    ) -> Result<Self, Self::TryMergeUnsafeError> {
        munmap(data.start_ptr(), usize::from(data.length_bytes().count))?;
        Ok(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unmapped1() {
        let pages_per_partition_usize = 128;
        let pages_per_partition = NonZeroUsize::new(pages_per_partition_usize).unwrap();

        let unmapped_anonymous_pages = UnmappedAnonymousPages::try_new_normal(Pages {
            count: pages_per_partition,
        })
        .unwrap();
        let (anonymous_pages, unmapped_anonymous_pages) =
            unmapped_anonymous_pages.try_partition().unwrap().into();
        let count = usize::from(anonymous_pages.length_bytes().count);
        assert_eq!(
            count,
            pages_per_partition_usize * usize::from(page_size_bytes().count)
        );

        let start_ptr = anonymous_pages.start_ptr() as *mut u8;

        unsafe {
            start_ptr.write_bytes(42, count);
        }
        for i in 0..count {
            assert_eq!(unsafe { *start_ptr.add(i) }, 42);
        }

        unsafe {
            unmapped_anonymous_pages
                .try_merge_unsafe(anonymous_pages)
                .unwrap();
        }
    }
}
