pub trait Partition<L, R> {
    fn partition(self) -> Partitioned<L, R>;
}

pub trait TryPartition<L, R, E> {
    fn try_partition(self) -> Result<Partitioned<L, R>, E>;
}

pub struct Partitioned<L, R> {
    left: L,
    right: R,
}

impl<L, R> Partitioned<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}