use std::alloc::Layout;

// use super::{block::{Block, Blk, Replace}, block_dynamic::DynamicBlock, map_dynamic_block::BlockMap};

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

    pub fn map<C, F: Fn(B) -> C>(self, f: F) -> Blueprinted<C> {
        Blueprinted::new(f(self.block), self.blueprint)
    }
}

// impl<B> Blk for Blueprinted<B> {
//     type B = B;
// }

// impl<B, X, Y> Replace<X, Y> for Blueprinted<X> {
//     fn replace<F: Fn(X) -> Y>(self, f: F) -> Self {
//         todo!()
//     }
// }

// impl<B> Block<B> for Blueprinted<B> {
//     fn map<C, F: Fn(B) -> C, CB: Block<C>>(self, f: F) -> CB {
//         Self::new(f(self.block), self.blueprint)
//     }
// }

// impl<R, F: Fn(DynamicBlock) -> R, B: MapDynamicBlock<R, F>> MapDynamicBlock<R, F>
//     for Blueprinted<B>
// {
//     fn map_dynamic(self, f: F) -> Self {
//         Self::new(f(self.block), self.blueprint)
//     }
// }
