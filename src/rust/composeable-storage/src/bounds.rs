// use num_traits::{CheckedAdd, CheckedSub, Zero};

// use crate::{
//     arithmetic_errors::Overflow,
//     partition::{Partitioned, TryPartition, TryPartitionTransform},
// };

// #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
// pub struct AtOrAbove<T>(T);

// #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
// pub struct AtOrBelow<T>(T);

// #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
// pub struct AtOrBetween<T: PartialOrd> {
//     // Invariant: at_or_above <= at_or_below
//     at_or_above: T,
//     at_or_below: T,
// }

// impl<T: PartialOrd> AtOrBetween<T> {
//     pub fn try_new(at_or_above: T, at_or_below: T) -> Option<Self> {
//         (at_or_above <= at_or_below).then(|| Self {
//             at_or_above,
//             at_or_below,
//         })
//     }
// }

// #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
// pub struct OffsetAtOrAbove<T: PartialOrd + Zero> {
//     // Invariant: offset >= 0
//     offset: T,
//     at_or_above: T,
// }

// impl<T: PartialOrd + Zero> OffsetAtOrAbove<T> {
//     pub fn try_new(offset: T, at_or_above: T) -> Option<Self> {
//         (offset >= T::zero()).then(|| Self {
//             offset,
//             at_or_above,
//         })
//     }
// }

// impl<T: PartialOrd + Zero> From<AtOrAbove<T>> for OffsetAtOrAbove<T> {
//     fn from(value: AtOrAbove<T>) -> Self {
//         Self {
//             offset: T::zero(),
//             at_or_above: value.0,
//         }
//     }
// }

// #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
// pub struct OffsetAtOrBelow<T: PartialOrd + Zero> {
//     // Invariant: offset >= 0
//     offset: T,
//     at_or_below: T,
// }

// impl<T: PartialOrd + Zero> OffsetAtOrBelow<T> {
//     pub fn try_new(offset: T, at_or_below: T) -> Option<Self> {
//         (offset >= T::zero()).then(|| Self {
//             offset,
//             at_or_below,
//         })
//     }
// }

// #[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
// pub struct OffsetAtOrBetween<T: PartialOrd + Zero + CheckedSub> {
//     // Invariant offset >= 0
//     // Invariant at_or_above <= at_or_below
//     // Invariant at_or_below.checked_sub(&at_or_above)? <= offset
//     offset: T,
//     at_or_above: T,
//     at_or_below: T,
// }

// impl<T: PartialOrd + Zero + CheckedSub> OffsetAtOrBetween<T> {
//     pub fn try_new(offset: T, at_or_above: T, at_or_below: T) -> Option<Self> {
//         (offset >= T::zero()
//             && at_or_above <= at_or_below
//             && (at_or_below.checked_sub(&at_or_above)? <= offset))
//             .then(|| Self {
//                 offset,
//                 at_or_above,
//                 at_or_below,
//             })
//     }
// }

// impl<T: PartialOrd + Zero + CheckedAdd + Clone> TryPartitionTransform<AtOrBetween<T>, AtOrAbove<T>>
//     for OffsetAtOrAbove<T>
// {
//     type TryPartitionIntoError = Overflow;

//     fn try_partition_transform(
//         self,
//     ) -> Result<Partitioned<AtOrBetween<T>, AtOrAbove<T>>, Self::TryPartitionIntoError> {
//         self.offset
//             .checked_add(&self.at_or_above)
//             .ok_or(Overflow)
//             .map(|v| {
//                 let data = AtOrBetween {
//                     at_or_above: self.at_or_above,
//                     at_or_below: v.clone(),
//                 };
//                 let storage = AtOrAbove(v);
//                 Partitioned::new(data, storage)
//             })
//     }
// }

// impl<T: PartialOrd + Zero + CheckedAdd + Clone> TryPartition<AtOrBetween<T>>
//     for OffsetAtOrAbove<T>
// {
//     type TryPartitionError = Overflow;

//     fn try_partition(self) -> Result<Partitioned<AtOrBetween<T>, Self>, Self::TryPartitionError> {
//         let (at_or_between, at_or_above) = self.try_partition_transform()?.into();
//         Ok(Partitioned::new(at_or_between, at_or_above.into()))
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn offset_at_or_above1() {
//         let x = OffsetAtOrAbove::try_new(16, 8).unwrap();
//         {
//             let (y, x) = x.try_partition_transform().unwrap().into();
//             assert_eq!(y, AtOrBetween::try_new(8, 24).unwrap());
//             assert_eq!(x, AtOrAbove(24));
//         }
//         let (y, x) = x.try_partition().unwrap().into();
//         assert_eq!(y, AtOrBetween::try_new(8, 24).unwrap());
//         assert_eq!(x, OffsetAtOrAbove::try_new(0, 24).unwrap());
//     }
// }
