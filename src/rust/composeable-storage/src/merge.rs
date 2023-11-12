use crate::partition::Partitioned;

pub trait TryMergeUnsafe<L, R, O, E> {
    unsafe fn try_merge(p: Partitioned<L, R>) -> Result<O, E>;
}
