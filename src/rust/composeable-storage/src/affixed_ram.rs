use std::alloc::Layout;

use crate::{
    affix::{Affixed, Affixer},
    partition::{Partitioned, TryPartition},
    slice::Ram,
};

pub struct AffixedRam(Affixed<Ram, Ram, Ram>);

pub enum AffixerPartitionError<E> {
    RamPartitionErrror(E),
    LayoutPartitionError,
}

impl<S, E, P: TryPartition<Ram, S, E>> TryPartition<AffixedRam, S, E> for P {
    fn try_partition(self) -> Result<Partitioned<AffixedRam, S>, E> {
        todo!()
    }
}

fn try_part<S, E, G: TryPartition<Ram, S, E>>(
    s: Affixer<G, Layout, Layout, Layout>,
) -> Result<Partitioned<AffixedRam, S>, E> {
    todo!()
}
