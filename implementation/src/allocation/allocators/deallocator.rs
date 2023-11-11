pub trait Deallocator<B> {
    type DeallocateError;
    unsafe fn deallocate(&self, block: B) -> Option<Self::DeallocateError>;
}
