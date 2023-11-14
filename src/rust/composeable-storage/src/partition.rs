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
}

impl<D, S> From<Partitioned<D, S>> for (D, S) {
    fn from(value: Partitioned<D, S>) -> Self {
        (value.data, value.storage)
    }
}