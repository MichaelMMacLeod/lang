use crate::merge::TryMergeUnchecked;

pub trait TryPartition<L, E>: Sized {
    fn try_partition(self) -> Result<Partitioned<L, Self>, E>;
}

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Partitioned<L, R> {
    left: L,
    right: R,
}

impl<L, R> Partitioned<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }

    pub fn left(&self) -> &L {
        &self.left
    }

    pub fn right(&self) -> &R {
        &self.right
    }

    pub fn as_tuple(self) -> (L, R) {
        (self.left, self.right)
    }

    pub fn transform<T, F>(self, f: F) -> T
    where
        F: FnOnce(L, R) -> T,
    {
        f(self.left, self.right)
    }
}

impl<L, R: TryMergeUnchecked<L>> Partitioned<L, R> {
    pub unsafe fn try_merge_unchecked(self) -> Result<R, R::MergeError> {
        self.right.try_merge_unchecked(self.left)
    }
}

// impl<L, E, M: TryMergeUnsafe<L, E>> Partitioned<L, M> {
//     unsafe fn try_merge(self) -> Result<L, E> {
//         TryMergeUnsafe::try_merge(self)
//     }
// }
