use num_traits::Zero;

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
    at_or_above: T,
    at_or_below: T,
}

impl<T: Boundable> AtOrInside<T> {
    pub fn try_new(at_or_above: T, at_or_below: T) -> Option<Self> {
        (at_or_above <= at_or_below).then(|| Self {
            at_or_above,
            at_or_below,
        })
    }
}

impl<T: Boundable> From<AtOrInside<T>> for (AtOrAbove<T>, AtOrBelow<T>) {
    fn from(value: AtOrInside<T>) -> Self {
        (AtOrAbove(value.at_or_above), AtOrBelow(value.at_or_below))
    }
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OffsetAtOrAbove<T: Boundable> {
    // invariant: offset >= at_or_above
    offset: T,
    at_or_above: T,
}

impl<T: Boundable> OffsetAtOrAbove<T> {
    pub fn try_new(offset: T, at_or_above: T) -> Option<Self> {
        (offset >= at_or_above).then(|| Self {
            offset,
            at_or_above,
        })
    }
}

impl<T: Boundable> From<OffsetAtOrAbove<T>> for (T, AtOrAbove<T>) {
    fn from(value: OffsetAtOrAbove<T>) -> Self {
        (value.offset, AtOrAbove(value.at_or_above))
    }
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct OffsetAtOrBelow<T: Boundable> {
    // invariant: offset <= at_or_below
    offset: T,
    at_or_below: T,
}

impl<T: Boundable> OffsetAtOrBelow<T> {
    pub fn try_new(offset: T, at_or_below: T) -> Option<Self> {
        (offset <= at_or_below).then(|| Self {
            offset,
            at_or_below,
        })
    }
}

impl<T: Boundable> From<OffsetAtOrBelow<T>> for (T, AtOrBelow<T>) {
    fn from(value: OffsetAtOrBelow<T>) -> Self {
        (value.offset, AtOrBelow(value.at_or_below))
    }
}

impl<T: PartialOrd + Zero + Copy> Partition<AtOrInside<T>> for OffsetAtOrAbove<T> {
    fn partition(self) -> Partitioned<AtOrInside<T>, Self> {
        let data = AtOrInside {
            at_or_above: self.at_or_above,
            at_or_below: self.offset,
        };
        let mut zero = self.offset.clone();
        zero.set_zero();
        let storage = OffsetAtOrAbove {
            at_or_above: self.offset,
            offset: zero,
        };
        Partitioned::new(data, storage)
    }
}

#[cfg(test)]
mod test {
    use crate::partition::Partition;

    use super::OffsetAtOrAbove;

    #[test]
    fn partition1() {
        let aoo1 = OffsetAtOrAbove::try_new(64, 0).unwrap();
        let (aoi, aoo2) = aoo1.partition().into();
        let (at_or_above, at_or_below) = aoi.into();
    }
}
