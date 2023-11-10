use super::{block_dynamic::DynamicBlock, map_dynamic_block::MapDynamicBlock};

pub struct Initialized<B> {
    block: B,
}

impl<B> Initialized<B> {
    pub fn new(block: B) -> Self {
        Self { block }
    }
}

impl<B: MapDynamicBlock> MapDynamicBlock for Initialized<B> {
    fn map<R, F: Fn(DynamicBlock) -> R>(self, f: F) -> Self {
        Self::new(self.block.map(f))
    }
}