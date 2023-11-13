use std::alloc::Layout;

use crate::{
    affix::{Affixed, Affixer},
    partition::{Partitioned, TryPartition},
    ram::Ram,
};

pub struct AffixedRam(Affixed<Ram, Ram, Ram>);

pub enum AffixerPartitionError<E> {
    RamPartitionErrror(E),
    LayoutPartitionError,
}