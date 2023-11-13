use crate::partition::Partitioned;

pub trait TryMergeUnchecked<L>: Sized {
    type MergeError;
    unsafe fn try_merge_unchecked(self, left: L) -> Result<Self, Self::MergeError>;
}
