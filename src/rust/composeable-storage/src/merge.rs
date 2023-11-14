pub trait TryMerge<Data>: Sized {
    type TryMergeError;
    fn try_merge(self, data: Data) -> Result<Self, Self::TryMergeError>;
}

pub trait TryMergeUnsafe<Data>: Sized {
    type TryMergeUnsafeError;
    unsafe fn try_merge_unsafe(self, data: Data) -> Result<Self, Self::TryMergeUnsafeError>;
}