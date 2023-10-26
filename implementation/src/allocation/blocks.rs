pub trait Block {
    fn start(&self) -> usize;
    fn num_bytes(&self) -> usize;
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct DynamicSizedBlock {
    start: usize,
    num_bytes: usize,
}

impl DynamicSizedBlock {
    pub fn new(start: usize, num_bytes: usize) -> Self {
        Self { start, num_bytes }
    }
}

impl Block for DynamicSizedBlock {
    fn start(&self) -> usize {
        self.start
    }

    fn num_bytes(&self) -> usize {
        self.num_bytes
    }
}

pub struct StaticSizedBlock<const NUM_BYTES: usize> {
    start: usize,
}

impl<const NUM_BYTES: usize> StaticSizedBlock<NUM_BYTES> {
    fn new(start: usize) -> Self {
        Self { start }
    }
}

impl<const NUM_BYTES: usize> Block for StaticSizedBlock<NUM_BYTES> {
    fn start(&self) -> usize {
        self.start
    }

    fn num_bytes(&self) -> usize {
        NUM_BYTES
    }
}