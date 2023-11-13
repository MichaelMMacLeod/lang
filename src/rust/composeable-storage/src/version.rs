use crate::partition::{Partitioned, TryPartition};

pub trait Version: Sized {
    fn first() -> Self;
    fn try_next(self) -> Option<Self>;
    fn next(self) -> Self;
}

impl<T: From<usize> + Into<usize>> Version for T {
    fn first() -> Self {
        0.into()
    }

    fn try_next(self) -> Option<Self> {
        self.into().checked_add(1).map(Into::into)
    }

    fn next(self) -> Self {
        self.into().wrapping_add(1).into()
    }
}

pub struct Versioned<V: Version, S> {
    version: V,
    storage: S,
}

impl<V: Version, S> Versioned<V, S> {
    pub fn new(storage: S) -> Self {
        Self {
            version: V::first(),
            storage,
        }
    }

    pub fn storage(&self) -> &S {
        &self.storage
    }

    pub fn transform<T, F: FnOnce(S) -> T>(self, f: F) -> T {
        f(self.storage)
    }
}

pub enum VersionedPartitionError<E> {
    NoNextVersion,
    StoragePartitionError(E),
}

// S: TryPartition<A, E>
// TryPartition<A, Versioned<V, VersionedPartitionError<E>>, E> for Versioned<V, S>

impl<A, E, S: TryPartition<A, S, E>, V: Version>
    TryPartition<A, Versioned<V, S>, VersionedPartitionError<E>> for Versioned<V, S>
{
    fn try_partition(self) -> Result<Partitioned<A, Versioned<V, S>>, VersionedPartitionError<E>> {
        todo!()
    }
}

// impl<E, V, S: TryPartition<V, S, VersionedPartitionError<E>>> TryPartition<R, Versioned<V, S>,
