pub trait Block {
    fn start(&self) -> usize;
    fn num_bits(&self) -> usize;
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct DynamicSizedBlock {
    start: usize,
    num_bits: usize,
}

impl DynamicSizedBlock {
    pub fn new(start: usize, num_bits: usize) -> Self {
        Self { start, num_bits }
    }
}

impl Block for DynamicSizedBlock {
    fn start(&self) -> usize {
        self.start
    }

    fn num_bits(&self) -> usize {
        self.num_bits
    }
}

pub struct StaticSizedBlock<const NUM_BITS: usize> {
    start: usize,
}

impl<const NUM_BITS: usize> StaticSizedBlock<NUM_BITS> {
    fn new(start: usize) -> Self {
        Self { start }
    }
}

impl<const NUM_BITS: usize> Block for StaticSizedBlock<NUM_BITS> {
    fn start(&self) -> usize {
        self.start
    }

    fn num_bits(&self) -> usize {
        NUM_BITS
    }
}