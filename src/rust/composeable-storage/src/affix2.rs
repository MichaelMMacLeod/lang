use crate::{
    aligned::{AlignedLength, AlignedLengthType},
    arithmetic_errors::Overflow,
    merge::TryMergeTransform,
    partition::{Partitioned, TryPartition},
};

pub struct Affixed2<T> {
    prefix: T,
    middle: T,
    suffix: T,
    combined: T,
}

pub struct Affixer<S, L: AlignedLengthType> {
    start_of_middle: L,
    start_of_suffix: L,
    length: L,
    storage: S,
}

impl<S, L: AlignedLengthType> Affixer<S, L> {
    pub fn try_new(
        storage: S,
        prefix: AlignedLength<L>,
        middle: AlignedLength<L>,
        suffix: AlignedLength<L>,
    ) -> Result<Self, Overflow> {
        let (start_of_middle, prefix_plus_middle) = prefix.try_merge_transform(middle)?.into();
        let (start_of_suffix, all_combined) = prefix_plus_middle.try_merge_transform(suffix)?.into();
        let length = all_combined.unaligned_length();
        Ok(Self {
            start_of_middle,
            start_of_suffix,
            length,
            storage,
        })
    }
}

impl<D, S: TryPartition<D>, L: AlignedLengthType> TryPartition<Affixed2<D>> for Affixer<S, L> {
    type TryPartitionError = S::TryPartitionError;

    fn try_partition(self) -> Result<Partitioned<Affixed2<D>, Self>, Self::TryPartitionError> {
        todo!()
    }
}
