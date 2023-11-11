use super::block_dynamic::DynamicBlock;

pub struct Initialized<B> {
    block: B,
}

impl<B> Initialized<B> {
    pub fn new(block: B) -> Self {
        Self { block }
    }

    pub fn block(self) -> B {
        self.block
    }

    pub fn map<C, F: Fn(B) -> C>(self, f: F) -> Initialized<C> {
        Initialized::new(f(self.block))
    }
}