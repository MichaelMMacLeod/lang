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

pub trait MergeInto<Data, New>: Sized {
    fn merge_into(self, data: Data) -> New;
}

pub trait TryMergeInto<Data, New>: Sized {
    type TryMergeIntoError;
    fn try_merge_into(self, data: Data) -> Result<New, Self::TryMergeIntoError>;
}

pub trait MergeIntoUnsafe<Data, New>: Sized {
    unsafe fn merge_into(self, data: Data) -> New;
}

pub trait TryMergeIntoUnsafe<Data, New>: Sized {
    type TryMergeIntoUnsafeError;
    unsafe fn try_merge_into(self, data: Data) -> Result<New, Self::TryMergeIntoUnsafeError>;
}