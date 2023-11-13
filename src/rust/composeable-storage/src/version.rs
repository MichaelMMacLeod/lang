use crate::partition::{Partitioned, TryPartition};

pub trait Version: Sized + PartialOrd + Copy {
    fn first() -> Self;
    fn try_next(self) -> Option<Self>;
    fn next(self) -> Self;
}

// This is adapted from 'new_key_type!' in the 'slotmap' crate.
macro_rules! define_usize_version {
    ( $(#[$outer:meta])* $vis:vis struct $name:ident; $($others:tt)* ) => {
        $(#[$outer])*
        #[derive(Copy, Clone, Default,
                 PartialEq, Eq, PartialOrd, Ord,
                 Hash, Debug)]
        #[repr(transparent)]
        $vis struct $name(usize);

        impl From<usize> for $name {
            fn from(value: usize) -> Self {
                $name(value)
            }
        }

        impl From<$name> for usize {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    }
}

impl<T: Copy + PartialOrd + From<usize> + Into<usize>> Version for T {
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

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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

    pub fn version(&self) -> &V {
        &self.version
    }

    pub fn storage(&self) -> &S {
        &self.storage
    }

    pub fn transform<T, F: FnOnce(S) -> T>(self, f: F) -> T {
        f(self.storage)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum VersionedPartitionError<E> {
    NoNextVersion,
    StoragePartitionError(E),
}

impl<E> From<E> for VersionedPartitionError<E> {
    fn from(value: E) -> Self {
        Self::StoragePartitionError(value)
    }
}

impl<A, E, S: TryPartition<A, E>, V: Version> TryPartition<A, VersionedPartitionError<E>>
    for Versioned<V, S>
{
    fn try_partition(self) -> Result<Partitioned<A, Versioned<V, S>>, VersionedPartitionError<E>> {
        self.storage.try_partition()?.transform(|a, storage| {
            self.version
                .try_next()
                .ok_or(VersionedPartitionError::NoNextVersion)
                .map(|version| Partitioned::new(a, Self { storage, version }))
        })
    }
}

#[cfg(test)]
mod test {
    use std::alloc::{Layout, System};

    use crate::unused_ram::UnusedRam;

    use super::*;

    #[test]
    fn version0() {
        define_usize_version! {
            struct V;
        }
        let v0: Versioned<V, _> = Versioned::new(UnusedRam::new(
            System,
            Layout::from_size_align(16, 64).unwrap(),
        ));
        let v0v = *v0.version();

        let (ram1, v1) = v0.try_partition().unwrap().as_tuple();
        let v1v = *v1.version();
        assert!(v0v < v1v);

        let (ram2, v2) = v1.try_partition().unwrap().as_tuple();
        let v2v = *v2.version();
        assert!(v1v < v2v);

        // v2.try_merge(ram1);
    }
}
