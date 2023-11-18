use std::{
    num::NonZeroUsize,
    ops::{Add, BitAnd, Not},
};

use num_derive::Num;
use num_traits::{CheckedAdd, Unsigned, Num};

use crate::arithmetic_errors::Overflow;

/// Represents a certain number of bytes.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Num)]
pub struct Bytes<T> {
    pub count: T,
}

impl<T: Add> Add for Bytes<T> {
    type Output = Bytes<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Bytes {
            count: self.count + rhs.count,
        }
    }
}

impl<T: CheckedAdd> CheckedAdd for Bytes<T> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        self.count.checked_add(&v.count).map(|count| Self { count })
    }
}

impl<T: Not> Not for Bytes<T> {
    type Output = Bytes<T::Output>;

    fn not(self) -> Self::Output {
        Self { count: !self.count }
    }
}

impl<T: BitAnd> BitAnd for Bytes<T> {
    type Output = Bytes<T::Output>;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            count: self.count & rhs.count,
        }
    }
}

impl<T: Num> Num for Bytes<T> {
    type FromStrRadixErr;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl<T: Unsigned> Unsigned for Bytes<T> {}

/// Represents a certain number of normal pages (these are usually 4KiB)
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Pages<T> {
    pub count: T,
}

pub(crate) fn page_size_bytes() -> Bytes<NonZeroUsize> {
    use std::sync::OnceLock;

    static PAGE_SIZE: OnceLock<NonZeroUsize> = OnceLock::new();
    let count = *PAGE_SIZE.get_or_init(|| {
        #[cfg(unix)]
        let count = {
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

            // Safety:
            //
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
        };

        // TODO: use 'winapi' crate to get actual value for windows

        count
    });
    Bytes { count }
}

impl TryFrom<Pages<NonZeroUsize>> for Bytes<NonZeroUsize> {
    type Error = Overflow;

    fn try_from(value: Pages<NonZeroUsize>) -> Result<Self, Self::Error> {
        let page_size_bytes = page_size_bytes();
        value
            .count
            .checked_mul(page_size_bytes.count)
            .ok_or(Overflow)
            .map(|b| Bytes { count: b })
    }
}
