use crate::{
    aligned::{AlignedLength, AlignedLengthType},
    arithmetic_errors::Overflow,
    lower_bounded::LowerBounded,
    merge::{TryMerge, TryMergeTransform},
    partition::{Partitioned, TryPartition, TryPartitionTransform},
    units::Bytes,
};

pub struct Affixed<L: AlignedLengthType> {
    prefix: AlignedLength<L>,
    middle: AlignedLength<L>,
    suffix: AlignedLength<L>,
    combined_length: L,
}

impl<L: AlignedLengthType> Affixed<L> {
    pub fn new(
        prefix: AlignedLength<L>,
        middle: AlignedLength<L>,
        suffix: AlignedLength<L>,
    ) -> Self {
        Self {
            prefix,
            middle,
            suffix,
            combined_length,
        }
    }
}

pub struct Affixer<S, L: AlignedLengthType> {
    start_of_middle: L,
    start_of_suffix: L,
    length: L,
    storage: S,
}

impl<S, L> TryMergeTransform<Affixed<L>> for S
where
    L: AlignedLengthType,
    S: TryPartitionTransform<AlignedLength<L>, LowerBounded<L, S>>,
{
    type TryMergeTransformError = Overflow;
    type New = Affixer<S, L>;

    fn try_merge_transform(
        self,
        data: Affixed<L>,
    ) -> Result<Self::New, Self::TryMergeTransformError> {
        todo!()
    }
}

impl<S, L: AlignedLengthType> Affixer<S, L>
where
    S: TryMergeTransform<AlignedLength<Bytes<usize>>, New = LowerBounded<Bytes<L>, S>>,
{
    pub fn try_new(
        storage: S,
        prefix: AlignedLength<L>,
        middle: AlignedLength<L>,
        suffix: AlignedLength<L>,
    ) -> Result<Self, Overflow> {
        let (start_of_middle, prefix_plus_middle) = prefix.try_merge_transform(middle)?.into();
        let (start_of_suffix, all_combined) =
            prefix_plus_middle.try_merge_transform(suffix)?.into();
        let length = all_combined.unaligned_length();
        Ok(Self {
            start_of_middle,
            start_of_suffix,
            length,
            storage,
        })
    }
}

impl<D, S: TryPartition<D>, L: AlignedLengthType> TryPartition<Affixed<D>> for Affixer<S, L> {
    type TryPartitionError = S::TryPartitionError;

    fn try_partition(self) -> Result<Partitioned<Affixed<D>, Self>, Self::TryPartitionError> {
        todo!()
    }
}
