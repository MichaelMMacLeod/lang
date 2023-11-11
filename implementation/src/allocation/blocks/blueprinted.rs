use std::alloc::Layout;

use super::block_dynamic::DynamicBlock;

// use super::{block::{Block, Blk, Replace}, block_dynamic::DynamicBlock, map_dynamic_block::BlockMap};

pub struct Blueprinted<B> {
    block: B,
    blueprint: Layout,
}

impl<B> Blueprinted<B> {
    pub fn new(block: B, blueprint: Layout) -> Self {
        Self { block, blueprint }
    }

    pub fn block(self) -> B {
        self.block
    }

    pub fn blueprint(&self) -> Layout {
        self.blueprint
    }

    pub fn map<C, F: Fn(B) -> C>(self, f: F) -> Blueprinted<C> {
        Blueprinted::new(f(self.block), self.blueprint)
    }
}

// impl From<Blueprinted<DynamicBlock>> for DynamicBlock {
//     fn from(value: Blueprinted<DynamicBlock>) -> Self {
//         value.block
//     }
// }