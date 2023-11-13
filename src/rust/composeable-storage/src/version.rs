use crate::{
    merge::TryMergeUnsafe,
    partition::{Partitioned, TryPartition},
};

pub trait Version: Sized + PartialOrd + Copy {
    fn first() -> Self;
    fn try_next(self) -> Option<Self>;
    fn previous(self) -> Self;
}

// This is adapted from 'new_key_type!' in the 'slotmap' crate.
#[macro_export]
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

    fn previous(self) -> Self {
        self.into().saturating_sub(1).into()
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

    pub fn as_tuple(self) -> (V, S) {
        (self.version, self.storage)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum VersionedPartitionError<E> {
    NoNextVersion,
    StorageError(E),
}

impl<E> From<E> for VersionedPartitionError<E> {
    fn from(value: E) -> Self {
        Self::StorageError(value)
    }
}

impl<A, S: TryPartition<A>, V: Version> TryPartition<Versioned<V, A>> for Versioned<V, S> {
    type TryPartitionError = VersionedPartitionError<S::TryPartitionError>;

    fn try_partition(self) -> Result<Partitioned<Versioned<V, A>, Self>, Self::TryPartitionError> {
        let (data, storage): (A, S) = self.storage.try_partition()?.as_tuple();
        self.version
            .try_next()
            .ok_or(VersionedPartitionError::NoNextVersion)
            .map(|version| {
                let data: Versioned<V, A> = Versioned {
                    storage: data,
                    version,
                };
                let storage: Versioned<V, S> = Versioned { storage, version };
                Partitioned::new(data, storage)
            })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum VersionedTryMergeUnsafeError<E> {
    WrongVersion,
    StorageError(E),
}

impl<E> From<E> for VersionedTryMergeUnsafeError<E> {
    fn from(value: E) -> Self {
        Self::StorageError(value)
    }
}

impl<Data, Storage: TryMergeUnsafe<Data>, V: Version> TryMergeUnsafe<Versioned<V, Data>>
    for Versioned<V, Storage>
{
    type TryMergeUnsafeError = VersionedTryMergeUnsafeError<Storage::TryMergeUnsafeError>;

    unsafe fn try_merge_unsafe(
        self,
        data: Versioned<V, Data>,
    ) -> Result<Self, Self::TryMergeUnsafeError> {
        if self.version != data.version {
            Err(VersionedTryMergeUnsafeError::WrongVersion)
        } else {
            let storage = self.storage.try_merge_unsafe(data.storage)?;
            Ok(Versioned {
                storage,
                version: self.version.previous(),
            })
        }
    }
}

// impl<S, V: Version> Versioned<V, S> {
//     pub unsafe fn try_merge_unsafe<D>(
//         self,
//         data: Versioned<V, D>,
//     ) -> Result<S, S::TryMergeUnsafeError>
//     where
//         S: TryMergeUnsafe<D>,
//     {
//         self.try_merge_unsafe(data)
//     }
// }

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
        let unused_ram_0: Versioned<V, _> = Versioned::new(UnusedRam::new(
            System,
            Layout::from_size_align(16, 64).unwrap(),
        ));
        let unused_ram_0_version_a = *unused_ram_0.version();

        let (ram_1, unused_ram_1) = unused_ram_0.try_partition().unwrap().as_tuple();
        let unused_ram_1_version_a = *unused_ram_1.version();
        assert!(unused_ram_0_version_a < unused_ram_1_version_a);

        let (ram_2, unused_ram_2) = unused_ram_1.try_partition().unwrap().as_tuple();
        let unused_ram_2_version_a = *unused_ram_2.version();
        assert!(unused_ram_1_version_a < unused_ram_2_version_a);

        let unused_ram_1 = unsafe { unused_ram_2.try_merge_unsafe(ram_2).unwrap() };
        let unused_ram_1_version_b = *unused_ram_1.version();
        assert_eq!(unused_ram_1_version_b, unused_ram_1_version_a);

        let unused_ram_0 = unsafe { unused_ram_1.try_merge_unsafe(ram_1).unwrap() };
        let unused_ram_0_version_b = *unused_ram_0.version();
        assert_eq!(unused_ram_0_version_a, unused_ram_0_version_b);
    }
}
