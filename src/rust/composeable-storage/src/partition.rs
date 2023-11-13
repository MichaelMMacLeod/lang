use crate::merge::TryMergeUnchecked;

pub trait TryPartition<Data>: Sized {
    type TryPartitionError;
    fn try_partition(self) -> Result<Partitioned<Data, Self>, Self::TryPartitionError>;
}

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

    pub fn transform<T, F>(self, f: F) -> T
    where
        F: FnOnce(D, S) -> T,
    {
        f(self.data, self.storage)
    }
}

impl<D, S: TryMergeUnchecked<D>> Partitioned<D, S> {
    pub unsafe fn try_merge_unchecked(self) -> Result<S, S::MergeError> {
        self.storage.try_merge_unchecked(self.data)
    }
}