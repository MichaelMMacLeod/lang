pub struct Affixed<P, M, S, C> {
    prefix: P,
    middle: M,
    suffix: S,
    combined: C,
}

impl<P, M, S, C> Affixed<P, M, S, C> {
    pub fn new(prefix: P, middle: M, suffix: S, combined: C) -> Self {
        Self {
            prefix,
            middle,
            suffix,
            combined,
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

    pub fn combined(&self) -> &C {
        &self.combined
    }
}
