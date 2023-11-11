use std::ops::Deref;

use super::{slice::SliceBlock, contains::Contains};

pub struct Initialized<B> {
    block: B,
}

impl<B> Initialized<B> {
    pub fn initialized(self) -> InitializedPart<B> {
        self.into()
    }

    pub fn initialized_ref(&self) -> &B {
        &self.block
    }
}

impl<B: Contains<SliceBlock>> Initialized<B> {
    pub unsafe fn with_constant(b: B, val: u8) -> Initialized<B> {
        Self {
            block: b.map_part(|s| {
                s.start_ptr().write_bytes(val, s.len());
                s
            }),
        }
    }
}

pub struct InitializedPart<B>(Initialized<B>);

impl<B> InitializedPart<B> {
    pub fn initialized(self) -> Initialized<B> {
        self.into()
    }
}

impl<B> From<Initialized<B>> for InitializedPart<B> {
    fn from(value: Initialized<B>) -> Self {
        InitializedPart(value)
    }
}

impl<B> From<InitializedPart<B>> for Initialized<B> {
    fn from(value: InitializedPart<B>) -> Self {
        value.0
    }
}

impl<B> Deref for InitializedPart<B> {
    type Target = B;

    fn deref(&self) -> &Self::Target {
        &self.0.block
    }
}

impl<K, B: Contains<K>> Contains<K> for InitializedPart<B> {
    fn map_part<F: FnOnce(K) -> K>(self, f: F) -> Self {
        InitializedPart(Initialized {
            block: self.0.block.map_part(f),
        })
    }
}

impl<B> Contains<Initialized<B>> for Initialized<B> {
    fn map_part<F: FnOnce(Initialized<B>) -> Initialized<B>>(self, f: F) -> Self {
        f(self)
    }
}