pub trait Merge<T> {
    type Error;
    unsafe fn merge(&mut self, value: T) -> Option<Self::Error>;
}