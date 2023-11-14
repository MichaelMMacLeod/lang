use num_traits::{Zero, Unsigned};

use crate::{
    merge::TryMerge,
    partition::{Partition, Partitioned, TryPartition},
};

pub trait Boundable: Copy + PartialOrd + Zero {}
impl<T: PartialOrd + Zero + Copy> Boundable for T {}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AtOrAbove<T: Boundable>(pub T);

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AtOrBelow<T: Boundable>(pub T);

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)] // no Default
pub struct AtOrInside<T: Boundable> {
    // invariant: at_or_above <= at_or_below
    at_or_above: AtOrAbove<T>,
    at_or_below: AtOrBelow<T>,
}

impl<T: Boundable> AtOrInside<T> {
    pub fn try_new(at_or_above: AtOrAbove<T>, at_or_below: AtOrBelow<T>) -> Option<Self> {
        (at_or_above.0 <= at_or_below.0).then(|| Self {
            at_or_above,
            at_or_below,
        })
    }
}

impl<T: Boundable> From<AtOrInside<T>> for (AtOrAbove<T>, AtOrBelow<T>) {
    fn from(value: AtOrInside<T>) -> Self {
        (value.at_or_above, value.at_or_below)
    }
}

pub trait Offsettable: Boundable + Unsigned {}
impl<O: Boundable + Unsigned> Offsettable for O {}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Offset<O: Offsettable>(pub O);

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OffsetAtOrAbove<O: Offsettable> {
    // invariant: offset >= at_or_above
    offset: Offset<O>,
    at_or_above: AtOrAbove<O>,
}

impl<O: Offsettable> OffsetAtOrAbove<O> {
    pub fn try_new(offset: Offset<O>, at_or_above: AtOrAbove<O>) -> Option<Self> {
        (offset.0 >= at_or_above.0).then(|| Self {
            offset,
            at_or_above,
        })
    }
}

impl<O: Offsettable> From<OffsetAtOrAbove<O>> for (Offset<O>, AtOrAbove<O>) {
    fn from(value: OffsetAtOrAbove<O>) -> Self {
        (value.offset, value.at_or_above)
    }
}

// #[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
// pub struct OffsetAtOrBelow<T: Boundable> {
//     // invariant: offset <= at_or_below
//     offset: T,
//     at_or_below: T,
// }

// impl<T: Boundable> OffsetAtOrBelow<T> {
//     pub fn try_new(offset: T, at_or_below: T) -> Option<Self> {
//         (offset <= at_or_below).then(|| Self {
//             offset,
//             at_or_below,
//         })
//     }
// }

// impl<T: Boundable> From<OffsetAtOrBelow<T>> for (T, AtOrBelow<T>) {
//     fn from(value: OffsetAtOrBelow<T>) -> Self {
//         (value.offset, AtOrBelow(value.at_or_below))
//     }
// }

// impl<T: PartialOrd + Zero + Copy> Partition<AtOrInside<T>> for OffsetAtOrAbove<T> {
//     fn partition(self) -> Partitioned<AtOrInside<T>, Self> {
//         let data = AtOrInside {
//             at_or_above: self.at_or_above,
//             at_or_below: self.offset,
//         };
//         let mut zero = self.offset.clone();
//         zero.set_zero();
//         let storage = OffsetAtOrAbove {
//             at_or_above: self.offset,
//             offset: zero,
//         };
//         Partitioned::new(data, storage)
//     }
// }

// #[cfg(test)]
// mod test {
//     use crate::partition::Partition;

//     use super::OffsetAtOrAbove;

//     #[test]
//     fn partition1() {
//         let aoo1 = OffsetAtOrAbove::try_new(64, 0).unwrap();
//         let (aoi, aoo2) = aoo1.partition().into();
//         let (at_or_above, at_or_below) = aoi.into();
//     }
// }
