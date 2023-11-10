use super::block_dynamic::DynamicBlock;

pub trait MapDynamicBlock<R> {
    fn map<F: Fn(DynamicBlock) -> R>(self, f: F) -> Self;
}