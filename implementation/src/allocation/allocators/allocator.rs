use crate::allocation::blocks::block_dynamic::DynamicBlock;

pub trait Allocator<L, B> {
    type AllocateError;
    fn allocate(&self, layout: L) -> Result<B, Self::AllocateError>;
}
