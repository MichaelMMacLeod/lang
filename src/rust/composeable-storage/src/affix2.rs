use crate::{
    aligned::{AlignedLength, AlignedLengthType},
    partition::{Partitioned, TryPartition},
};

pub struct Affixed<D> {
    prefix: D,
    middle: D,
    suffix: D,
    combined: D,
}

pub struct Affixer<S, L: AlignedLengthType> {
    storage: S,
    prefix_offset: L,
    suffix_offset: L,
    combined_length: L,
}

pub enum TryPartitionAffixerError<S, D> {
    Storage(S),
    Data(D),
}

impl<L, D, S> TryPartition<Affixed<D>> for Affixer<S, L>
where
    L: AlignedLengthType,
    D: TryPartition<D, Selector = L> + Clone,
    S: TryPartition<D, Selector = AlignedLength<L>>,
{
    type Selector = S::Selector;
    type Error = TryPartitionAffixerError<S::Error, D::Error>;

    fn try_partition(
        self,
        selector: &Self::Selector,
    ) -> Result<Partitioned<Affixed<D>, Self>, Self::Error> {
        match self.storage.try_partition(selector) {
            Err(e) => Err(TryPartitionAffixerError::Storage(e)),
            Ok(partition) => {
                let (combined, storage) = partition.into();
                match combined.clone().try_partition(&self.suffix_offset) {
                    Err(e) => Err(TryPartitionAffixerError::Data(e)),
                    Ok(suffix_partition) => {
                        let (prefix_plus_middle, suffix) = suffix_partition.into();
                        match prefix_plus_middle.try_partition(&self.prefix_offset) {
                            Err(e) => Err(TryPartitionAffixerError::Data(e)),
                            Ok(middle_partition) => {
                                let (prefix, middle) = middle_partition.into();
                                Ok(Partitioned::new(
                                    Affixed {
                                        prefix,
                                        middle,
                                        suffix,
                                        combined,
                                    },
                                    Affixer { storage, ..self },
                                ))
                            }
                        }
                    }
                }
            }
        }
    }
}

fn test_affixer_1() {
    // let a = Affixer::
}

// use crate::{
//     aligned::{AlignedLength, AlignedLengthType},
//     arithmetic_errors::Overflow,
//     lower_bounded::LowerBounded,
//     merge::{TryMerge, TryMergeTransform},
//     partition::{Partitioned, TryPartition},
//     units::Bytes,
// };

// pub struct Affixed<L: AlignedLengthType> {
//     prefix: AlignedLength<L>,
//     middle: AlignedLength<L>,
//     suffix: AlignedLength<L>,
//     combined_length: L,
// }

// impl<L: AlignedLengthType> Affixed<L> {
//     pub fn new(
//         prefix: AlignedLength<L>,
//         middle: AlignedLength<L>,
//         suffix: AlignedLength<L>,
//     ) -> Self {
//         Self {
//             prefix,
//             middle,
//             suffix,
//             combined_length: todo!(),
//         }
//     }
// }

// pub struct Affixer<S, L: AlignedLengthType> {
//     start_of_middle: L,
//     start_of_suffix: L,
//     length: L,
//     storage: S,
// }

// // impl<S, L> TryMergeTransform<Affixed<L>> for S
// // where
// //     L: AlignedLengthType,
// //     S: TryPartitionTransform<AlignedLength<L>, LowerBounded<L, S>>,
// // {
// //     type TryMergeTransformError = Overflow;
// //     type New = Affixer<S, L>;

// //     fn try_merge_transform(
// //         self,
// //         data: Affixed<L>,
// //     ) -> Result<Self::New, Self::TryMergeTransformError> {
// //         todo!()
// //     }
// // }

// impl<S, L: AlignedLengthType> Affixer<S, L>
// where
//     S: TryMergeTransform<AlignedLength<Bytes<usize>>, New = LowerBounded<Bytes<L>, S>>,
// {
//     pub fn try_new(
//         storage: S,
//         prefix: AlignedLength<L>,
//         middle: AlignedLength<L>,
//         suffix: AlignedLength<L>,
//     ) -> Result<Self, Overflow> {
//         let (start_of_middle, prefix_plus_middle) = prefix.try_merge_transform(middle)?.into();
//         let (start_of_suffix, all_combined) =
//             prefix_plus_middle.try_merge_transform(suffix)?.into();
//         let length = all_combined.unaligned_length();
//         Ok(Self {
//             start_of_middle,
//             start_of_suffix,
//             length,
//             storage,
//         })
//     }
// }

// // impl<D, S: TryPartition<D>, L: AlignedLengthType> TryPartition<Affixed<D>> for Affixer<S, L> {
// //     type TryPartitionError = S::TryPartitionError;

// //     fn try_partition(self) -> Result<Partitioned<Affixed<D>, Self>, Self::TryPartitionError> {
// //         todo!()
// //     }
// // }
