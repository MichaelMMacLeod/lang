pub trait Deallocator<B> {
    type DeallocateError;
    fn deallocate(&self, block: B) -> Option<Self::DeallocateError>;
}
