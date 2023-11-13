use crate::{
    alignment::Alignment,
    partition::{Partitioned, TryPartition},
};

pub struct AlignedSize {
    size: usize,
    alignment: Alignment,
}

impl AlignedSize {
    pub fn new(size: usize, alignment: Alignment) -> Self {
        Self { size, alignment }
    }
}

pub struct OffsetAlignedSize {
    // Invariant: offset <= aligned_size.size
    aligned_size: AlignedSize,
    offset: usize,
}

impl OffsetAlignedSize {
    pub fn try_new(aligned_size: AlignedSize, offset: usize) -> Option<Self> {
        (offset <= aligned_size.size).then(|| Self {
            aligned_size,
            offset,
        })
    }
}

struct E;

impl TryPartition<AlignedSize, AlignedSize, E> for OffsetAlignedSize {
    fn try_partition(self) -> Result<Partitioned<AlignedSize, AlignedSize>, E> {
        todo!()
    }
}
