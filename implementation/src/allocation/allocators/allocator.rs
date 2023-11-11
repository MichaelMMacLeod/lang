pub trait Allocator<L, B> {
    type AllocateError;
    fn allocate(&self, layout: L) -> Result<B, Self::AllocateError>;
}
