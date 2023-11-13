use crate::{
    alignment::Alignment,
    partition::{Partitioned, TryPartition},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Bytes(usize);

impl From<usize> for Bytes {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Bytes> for usize {
    fn from(value: Bytes) -> Self {
        value.0
    }
}

pub struct AlignedOffsetBytes {
    bytes: usize,
    alignment: Alignment,
}

enum AlignedOffsetBytesPartitionError {
    Underflow,
    Overflow,
}

impl TryPartition<Bytes, Bytes, AlignedOffsetBytesPartitionError> for AlignedOffsetBytes {
    fn try_partition(self) -> Result<Partitioned<Bytes, Bytes>, AlignedOffsetBytesPartitionError> {
        todo!()
    }
}
