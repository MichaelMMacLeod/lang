pub trait Merge<Data>: Sized {
    fn merge(self, data: Data) -> Self;
}

pub trait TryMerge<Data>: Sized {
    type TryMergeError;
    fn try_merge(self, data: Data) -> Result<Self, Self::TryMergeError>;
}

pub trait MergeUnsafe<Data>: Sized {
    unsafe fn merge_unsafe(self, data: Data) -> Self;
}

pub trait TryMergeUnsafe<Data>: Sized {
    type TryMergeUnsafeError;
    unsafe fn try_merge_unsafe(self, data: Data) -> Result<Self, Self::TryMergeUnsafeError>;
}

pub trait MergeTransform<Data>: Sized {
    type New;
    fn merge_transform(self, data: Data) -> Self::New;
}

pub trait TryMergeTransform<Data>: Sized {
    type TryMergeTransformError;
    type New;
    fn try_merge_transform(self, data: Data) -> Result<Self::New, Self::TryMergeTransformError>;
}

pub trait MergeTransformUnsafe<Data>: Sized {
    type New;
    unsafe fn merge_transform(self, data: Data) -> Self::New;
}

pub trait TryMergeTransformUnsafe<Data>: Sized {
    type TryMergeTransformUnsafeError;
    type New;
    unsafe fn try_merge_transform(
        self,
        data: Data,
    ) -> Result<Self::New, Self::TryMergeTransformUnsafeError>;
}
