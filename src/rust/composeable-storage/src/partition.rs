// pub trait Partition<Data>: Sized {
//     fn partition(self) -> Partitioned<Data, Self>;
// }

pub trait TryPartition<Data>: Sized {
    type Selector;
    type Error;
    fn try_partition(
        self,
        selector: &Self::Selector,
    ) -> Result<Partitioned<Data, Self>, Self::Error>;
}

// pub trait PartitionTransform<Data, New>: Sized {
//     fn partition_transform(self) -> Partitioned<Data, New>;
// }

// pub trait TryPartitionTransform<Data, New>: Sized {
//     type TryPartitionIntoError;
//     fn try_partition_transform(self)
//         -> Result<Partitioned<Data, New>, Self::TryPartitionIntoError>;
// }

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
