use std::ops::{BitAnd, Not};

use num_traits::{CheckedAdd, Unsigned, WrappingAdd};

use crate::{
    alignment::Alignment,
    arithmetic_errors::Overflow,
    merge::{TryMerge, TryMergeInto},
};

pub trait AlignedLengthType:
    Clone
    + TryFrom<usize>
    + PartialOrd
    + CheckedAdd
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + Unsigned
{
}
impl<T> AlignedLengthType for T where
    T: Clone
        + TryFrom<usize>
        + PartialOrd
        + CheckedAdd
        + Not<Output = Self>
        + BitAnd<Output = Self>
        + Unsigned
{
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AlignedLength<T: AlignedLengthType> {
    alignment: Alignment,
    length: T,
}

impl<T: AlignedLengthType> AlignedLength<T> {
    pub fn new(alignment: Alignment, length: T) -> Self {
        Self { alignment, length }
    }

    pub fn alignment(&self) -> &Alignment {
        &self.alignment
    }

    pub fn length(&self) -> &T {
        &self.length
    }

    /// Returns the length rounded up to the nearest multiple of the
    /// alignment.
    pub fn try_aligned_length(&self) -> Result<T, Overflow> {
        // This uses some bitwise tricks I learned from
        // https://doc.rust-lang.org/src/core/alloc/layout.rs.html#260
        T::try_from(usize::from(self.alignment).wrapping_sub(1))
            .ok()
            .and_then(|a| self.length.checked_add(&a).map(|l| l & !a))
            .ok_or(Overflow)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OffsetAlignedLength<T: AlignedLengthType> {
    // Invariant: offset <= length
    offset: T,
    alignment: Alignment,
    length: T,
}

impl<T: AlignedLengthType> OffsetAlignedLength<T> {
    pub fn try_new(offset: T, alignment: Alignment, length: T) -> Option<Self> {
        (offset <= length).then(|| Self {
            offset,
            alignment,
            length,
        })
    }

    pub fn offset(&self) -> &T {
        &self.offset
    }

    pub fn alignment(&self) -> &Alignment {
        &self.alignment
    }

    pub fn length(&self) -> &T {
        &self.length
    }
}

impl<T: AlignedLengthType> TryMergeInto<AlignedLength<T>, OffsetAlignedLength<T>>
    for AlignedLength<T>
{
    type TryMergeIntoError = Overflow;

    /// This is semantically the same as
    /// [`std::alloc::Layout::extend`].
    fn try_merge_into(
        self,
        data: AlignedLength<T>,
    ) -> Result<OffsetAlignedLength<T>, Self::TryMergeIntoError> {
        let alignment = self.alignment.max(data.alignment);
        let offset = AlignedLength::new(alignment, self.length).try_aligned_length()?;
        let length = offset.checked_add(&data.length).ok_or(Overflow)?;
        Ok(OffsetAlignedLength {
            offset,
            alignment,
            length,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        aligned::OffsetAlignedLength, alignment::Alignment, arithmetic_errors::Overflow,
        merge::TryMergeInto,
    };

    use super::AlignedLength;

    #[test]
    fn aligned_length1() {
        let a = AlignedLength::new(Alignment::new(1024).unwrap(), 64usize);
        assert_eq!(a.try_aligned_length(), Ok(1024));

        let a = AlignedLength::new(Alignment::new(1024).unwrap(), 1025usize);
        assert_eq!(a.try_aligned_length(), Ok(2048));

        let a = AlignedLength::new(Alignment::new(1024).unwrap(), 2048usize);
        assert_eq!(a.try_aligned_length(), Ok(2048));

        let a = AlignedLength::new(Alignment::new(1024).unwrap(), 64u8);
        assert_eq!(a.try_aligned_length(), Err(Overflow));
    }

    #[test]
    fn aligned_merge1() {
        let a1 = AlignedLength::new(Alignment::new(8).unwrap(), 1usize);
        let a2 = AlignedLength::new(Alignment::new(64).unwrap(), 32usize);
        assert_eq!(
            a1.try_merge_into(a2),
            Ok(OffsetAlignedLength::try_new(64, Alignment::new(64).unwrap(), 96).unwrap())
        );

        let a1 = AlignedLength::new(Alignment::new(64).unwrap(), 32usize);
        let a2 = AlignedLength::new(Alignment::new(8).unwrap(), 1usize);
        assert_eq!(
            a1.try_merge_into(a2),
            Ok(OffsetAlignedLength::try_new(64, Alignment::new(64).unwrap(), 65).unwrap())
        );

        let a1 = AlignedLength::new(Alignment::new(128).unwrap(), 64usize);
        let a2 = AlignedLength::new(Alignment::new(128).unwrap(), 64usize);
        assert_eq!(
            a1.try_merge_into(a2),
            Ok(OffsetAlignedLength::try_new(128, Alignment::new(128).unwrap(), 128 + 64).unwrap())
        );
    }
}
