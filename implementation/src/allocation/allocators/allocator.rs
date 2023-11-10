use std::alloc::Layout;

pub trait Allocator<B> {
    type AllocateError;
    fn allocate(&self, layout: Layout) -> Result<B, Self::AllocateError>;
}
