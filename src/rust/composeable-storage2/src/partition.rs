pub trait Partition<T> {
    type Error;
    type Selector;
    fn partition(&mut self, selector: Self::Selector) -> Result<T, Self::Error>;
}