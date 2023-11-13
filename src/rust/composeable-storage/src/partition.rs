pub trait TryPartition<L, E>: Sized {
    fn try_partition(self) -> Result<Partitioned<L, Self>, E>;
}

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
