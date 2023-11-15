use std::{ops::{Add, Deref}, num::NonZeroUsize};

use num_traits::Zero;

use crate::{
    alignment::Alignment,
    partition::{Partitioned, TryPartition},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Bytes<T>(pub T);

impl Bytes<NonZeroUsize> {
    pub fn non_zero_usize(self) -> NonZeroUsize {
        self.0
    }

    pub fn usize(self) -> usize {
        usize::from(self.0)
    }
}

impl<T: Add> Add for Bytes<T> {
    type Output = Bytes<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Bytes(self.0 + rhs.0)
    }
}

impl<T: Zero> Zero for Bytes<T> {
    fn zero() -> Self {
        Bytes(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

// pub struct AlignedOffsetBytes {
//     alignment: Alignment,
//     offset: Bytes,
//     bytes: Bytes,
// }

// enum AlignedOffsetBytesPartitionError {
//     Underflow,
//     Overflow,
// }

// impl TryPartition<Bytes, AlignedOffsetBytesPartitionError> for AlignedOffsetBytes {
//     fn try_partition(self) -> Result<Partitioned<Bytes, Self>, AlignedOffsetBytesPartitionError> {
//         todo!()
//     }
// }
