use crate::partition::Partitioned;

pub trait TryMergeUnsafe<L, E>: Sized {
    unsafe fn try_merge(p: Partitioned<L, Self>) -> Result<Self, E>;
}
