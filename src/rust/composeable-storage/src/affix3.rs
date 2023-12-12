use crate::{
    aligned::{AlignedLength, AlignedLengthType},
    alignment::Alignment,
    merge::TryMergeTransform,
    partition::{Partitioned, TryPartition},
};

pub struct Prefixed<D> {
    prefix: D,
    data: D,
}

pub struct Prefixer<S, T: AlignedLengthType> {
    storage: S,
    prefix: AlignedLength<T>,
}

pub enum TryPartitionAffixerError<S, D> {
    Storage(S),
    Data(D),
    Overflow,
}

impl<D, S, T> TryPartition<Prefixed<D>> for Prefixer<S, T>
where
    T: AlignedLengthType,
    S: TryPartition<D, Selector = AlignedLength<T>> + Clone,
    D: TryPartition<D, Selector = T>,
{
    type Selector = AlignedLength<T>;

    type Error = ();

    fn try_partition(
        self,
        selector: &Self::Selector,
    ) -> Result<Partitioned<Prefixed<D>, Self>, Self::Error> {
        match selector
            .clone()
            .try_merge_transform(self.prefix)
            .map(Into::<(T, AlignedLength<T>)>::into)
        {
            Ok((offset, aligned_length)) => {
                
            },
            Err(o) => Err(()),
        }
    }
}

pub struct Affixed<D> {
    prefix: D,
    middle: D,
    suffix: D,
    combined: D,
}

pub struct Affixer<S, T: AlignedLengthType> {
    storage: S,
    prefix: AlignedLength<T>,
    suffix: AlignedLength<T>,
}

impl<D, S, T> TryPartition<Affixed<D>> for Affixer<S, T>
where
    T: AlignedLengthType,
    S: TryPartition<D, Selector = AlignedLength<T>>,
    D: TryPartition<D, Selector = T>,
{
    type Selector = AlignedLength<T>;

    type Error = ();

    fn try_partition(
        self,
        selector: &Self::Selector,
    ) -> Result<Partitioned<Affixed<D>, Self>, Self::Error> {
        todo!()
    }
}
