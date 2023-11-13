use crate::merge::TryMergeUnsafe;

pub trait TryPartition<Data>: Sized {
    type TryPartitionError;
    fn try_partition(self) -> Result<Partitioned<Data, Self>, Self::TryPartitionError>;
}

#[must_use]
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Partitioned<Data, Storage> {
    data: Data,
    storage: Storage,
}

impl<D, S> Partitioned<D, S> {
    pub fn new(data: D, storage: S) -> Self {
        Self { data, storage }
    }

    pub fn data(&self) -> &D {
        &self.data
    }

    pub fn storage(&self) -> &S {
        &self.storage
    }

    pub fn as_tuple(self) -> (D, S) {
        (self.data, self.storage)
    }
}

impl<D, S: TryMergeUnsafe<D>> Partitioned<D, S> {
    pub unsafe fn try_merge_unsafe(self) -> Result<S, S::TryMergeUnsafeError> {
        self.storage.try_merge_unsafe(self.data)
    }
}