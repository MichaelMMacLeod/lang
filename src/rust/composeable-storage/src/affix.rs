use std::alloc::Layout;

pub trait TryAffix<P, M, S, E> {
    fn affix(self) -> Result<Affixed<P, M, S>, E>;
}

pub struct Affixed<P, M, S> {
    prefix: P,
    middle: M,
    suffix: S,
}

impl<P, M, S> Affixed<P, M, S> {
    pub fn new(prefix: P, middle: M, suffix: S) -> Self {
        Self {
            prefix,
            middle,
            suffix,
        }
    }

    pub fn prefix(&self) -> &P {
        &self.prefix
    }

    pub fn middle(&self) -> &M {
        &self.middle
    }

    pub fn suffix(&self) -> &S {
        &self.suffix
    }
}

pub type AffixedLayouts = Affixed<Layout, Layout, Layout>;

pub struct Affixer<Slice, P, M, S> {
    slice: Slice,
    affixed: Affixed<P, M, S>,
}