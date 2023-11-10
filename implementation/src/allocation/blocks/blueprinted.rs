use std::alloc::Layout;

use super::{block_dynamic::DynamicBlock, map_dynamic_block::MapDynamicBlock};

pub struct Blueprinted<B> {
    block: B,
    blueprint: Layout,
}

impl<B> Blueprinted<B> {
    pub fn new(block: B, blueprint: Layout) -> Self {
        Self { block, blueprint }
    }

    pub fn block(&self) -> &B {
        &self.block
    }

    pub fn blueprint(&self) -> Layout {
        self.blueprint
    }
}

impl<R, B: MapDynamicBlock<R>> MapDynamicBlock<R> for Blueprinted<B> {
    fn map<F: Fn(DynamicBlock) -> R>(self, f: F) -> Self {
        Self::new(self.block.map(f), self.blueprint)
    }
}
