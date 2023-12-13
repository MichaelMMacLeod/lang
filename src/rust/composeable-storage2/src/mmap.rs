use std::{ffi::c_void, num::NonZeroUsize};

use rustix::{
    io::Errno,
    mm::{mmap_anonymous, MapFlags, ProtFlags, munmap},
};

use crate::{partition::Partition, merge::Merge};

pub struct AnonymousPages {
    start_ptr: *mut c_void,
    length_bytes: NonZeroUsize,
}

fn page_size_bytes() -> NonZeroUsize {
    use std::sync::OnceLock;

    static PAGE_SIZE: OnceLock<NonZeroUsize> = OnceLock::new();
    *PAGE_SIZE.get_or_init(|| {
        // Does it make sense to use a OnceLock? What if the page
        // size changes?
        //
        // IEEE Std 1003.1-2017 (aka POSIX) (2017)
        // Section XSH sysconf()
        //
        //   sysconf() shall return the current variable value
        //   on the system. ... The value shall not change
        //   during the lifetime of the calling process.
        //
        // rustix::param::page_size is implemented in terms of
        // sysconf(_SC_PAGESIZE). Because sysconf on _SC_PAGESIZE
        // won't change while the process is running, we only need
        // to call this once, hence the OnceLock.
        let count: usize = rustix::param::page_size();

        // IEEE Std 1003.1-2017 (aka POSIX) (2017)
        // Section XBD headers <limits.h>
        //
        //   {PAGESIZE}
        //     Size in bytes of a page.
        //     Minimum Acceptable Value: 1
        //
        // We can assume that rustix::param::page_size() is
        // nonzero.
        unsafe { NonZeroUsize::new_unchecked(count) }
    })
}

pub struct Mmap;

impl Partition<AnonymousPages> for Mmap {
    type Error = Errno;

    type Selector = NonZeroUsize;

    fn partition(&mut self, page_count: Self::Selector) -> Result<AnonymousPages, Self::Error> {
        let length_bytes = page_count.saturating_mul(page_size_bytes());
        let start_ptr = unsafe {
            mmap_anonymous(
                std::ptr::null_mut(),
                length_bytes.get(),
                ProtFlags::READ | ProtFlags::WRITE,
                MapFlags::PRIVATE,
            )?
        };
        Ok(AnonymousPages {
            start_ptr,
            length_bytes,
        })
    }
}

impl Merge<AnonymousPages> for Mmap {
    type Error = Errno;

    unsafe fn merge(&mut self, value: AnonymousPages) -> Option<Self::Error> {
        munmap(value.start_ptr, value.length_bytes.into()).err()
    }
}