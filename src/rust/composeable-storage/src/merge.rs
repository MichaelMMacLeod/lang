pub trait TryMergeUnsafe<Data>: Sized {
    type TryMergeUnsafeError;
    unsafe fn try_merge_unsafe(self, data: Data) -> Result<Self, Self::TryMergeUnsafeError>;
}
