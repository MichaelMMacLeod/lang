use super::block_dynamic::DynamicBlock;

pub struct Initialized<B> {
    block: B,
}

impl<B> Initialized<B> {
    pub fn new(block: B) -> Self {
        Self { block }
    }

    pub fn map<C, F: Fn(B) -> C>(self, f: F) -> Initialized<C> {
        Initialized::new(f(self.block))
    }
}

impl Initialized<DynamicBlock> {
    pub fn ptr(&self) -> *mut [u8] {
        self.block.ptr()
    }

    pub fn start_ptr(&self) -> *mut u8 {
        self.block.start_ptr()
    }

    pub fn len(&self) -> usize {
        self.block.len()
    }
}