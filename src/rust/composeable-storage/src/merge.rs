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

pub trait MergeTransform<Data, New>: Sized {
    fn merge_transform(self, data: Data) -> New;
}

pub trait TryMergeTransform<Data, New>: Sized {
    type TryMergeTransformError;
    fn try_merge_transform(self, data: Data) -> Result<New, Self::TryMergeTransformError>;
}

pub trait MergeTransformUnsafe<Data, New>: Sized {
    unsafe fn merge_transform(self, data: Data) -> New;
}

pub trait TryMergeTransformUnsafe<Data, New>: Sized {
    type TryMergeTransformUnsafeError;
    unsafe fn try_merge_transform(
        self,
        data: Data,
    ) -> Result<New, Self::TryMergeTransformUnsafeError>;
}
