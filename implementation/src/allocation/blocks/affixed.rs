use std::ops::Deref;

use super::{contains::Contains, slice::SliceBlock};

pub struct Affixed<P, M, S, C> {
    prefix: P,
    middle: M,
    suffix: S,
    combined: C,
}

impl<P, M, S, C> Affixed<P, M, S, C> {
    pub fn prefix(self) -> PrefixPart<P, M, S, C> {
        self.into()
    }

    pub fn middle(self) -> MiddlePart<P, M, S, C> {
        self.into()
    }

    pub fn suffix(self) -> SuffixPart<P, M, S, C> {
        self.into()
    }

    pub fn combined(self) -> CombinedPart<P, M, S, C> {
        self.into()
    }

    pub fn prefix_ref(&self) -> &P {
        &self.prefix
    }

    pub fn middle_ref(&self) -> &M {
        &self.middle
    }

    pub fn suffix_ref(&self) -> &S {
        &self.suffix
    }

    pub fn combined_ref(&self) -> &C {
        &self.combined
    }
}

pub struct PrefixPart<P, M, S, C>(Affixed<P, M, S, C>);
pub struct MiddlePart<P, M, S, C>(Affixed<P, M, S, C>);
pub struct SuffixPart<P, M, S, C>(Affixed<P, M, S, C>);
pub struct CombinedPart<P, M, S, C>(Affixed<P, M, S, C>);

impl<P, M, S, C> PrefixPart<P, M, S, C> {
    pub fn affixed(self) -> Affixed<P, M, S, C> {
        self.into()
    }
}
impl<P, M, S, C> MiddlePart<P, M, S, C> {
    pub fn affixed(self) -> Affixed<P, M, S, C> {
        self.into()
    }
}
impl<P, M, S, C> SuffixPart<P, M, S, C> {
    pub fn affixed(self) -> Affixed<P, M, S, C> {
        self.into()
    }
}
impl<P, M, S, C> CombinedPart<P, M, S, C> {
    pub fn affixed(self) -> Affixed<P, M, S, C> {
        self.into()
    }
}

impl<P, M, S, C> From<Affixed<P, M, S, C>> for PrefixPart<P, M, S, C> {
    fn from(value: Affixed<P, M, S, C>) -> Self {
        Self(value)
    }
}
impl<P, M, S, C> From<Affixed<P, M, S, C>> for MiddlePart<P, M, S, C> {
    fn from(value: Affixed<P, M, S, C>) -> Self {
        Self(value)
    }
}
impl<P, M, S, C> From<Affixed<P, M, S, C>> for SuffixPart<P, M, S, C> {
    fn from(value: Affixed<P, M, S, C>) -> Self {
        Self(value)
    }
}
impl<P, M, S, C> From<Affixed<P, M, S, C>> for CombinedPart<P, M, S, C> {
    fn from(value: Affixed<P, M, S, C>) -> Self {
        Self(value)
    }
}

impl<P, M, S, C> From<PrefixPart<P, M, S, C>> for Affixed<P, M, S, C> {
    fn from(value: PrefixPart<P, M, S, C>) -> Self {
        value.0
    }
}
impl<P, M, S, C> From<MiddlePart<P, M, S, C>> for Affixed<P, M, S, C> {
    fn from(value: MiddlePart<P, M, S, C>) -> Self {
        value.0
    }
}
impl<P, M, S, C> From<SuffixPart<P, M, S, C>> for Affixed<P, M, S, C> {
    fn from(value: SuffixPart<P, M, S, C>) -> Self {
        value.0
    }
}
impl<P, M, S, C> From<CombinedPart<P, M, S, C>> for Affixed<P, M, S, C> {
    fn from(value: CombinedPart<P, M, S, C>) -> Self {
        value.0
    }
}

impl<P, M, S, C> Deref for PrefixPart<P, M, S, C> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.0.prefix
    }
}
impl<P, M, S, C> Deref for MiddlePart<P, M, S, C> {
    type Target = M;

    fn deref(&self) -> &Self::Target {
        &self.0.middle
    }
}
impl<P, M, S, C> Deref for SuffixPart<P, M, S, C> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.0.suffix
    }
}
impl<P, M, S, C> Deref for CombinedPart<P, M, S, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.0.combined
    }
}

impl<K, P: Contains<K>, M, S, C> Contains<K> for PrefixPart<P, M, S, C> {
    fn map_part<F: FnOnce(K) -> K>(self, f: F) -> Self {
        PrefixPart(Affixed {
            prefix: self.0.prefix.map_part(f),
            ..self.0
        })
    }
}
impl<K, P, M: Contains<K>, S, C> Contains<K> for MiddlePart<P, M, S, C> {
    fn map_part<F: FnOnce(K) -> K>(self, f: F) -> Self {
        MiddlePart(Affixed {
            middle: self.0.middle.map_part(f),
            ..self.0
        })
    }
}
impl<K, P, M, S: Contains<K>, C> Contains<K> for SuffixPart<P, M, S, C> {
    fn map_part<F: FnOnce(K) -> K>(self, f: F) -> Self {
        SuffixPart(Affixed {
            suffix: self.0.suffix.map_part(f),
            ..self.0
        })
    }
}
impl<K, P, M, S, C: Contains<K>> Contains<K> for CombinedPart<P, M, S, C> {
    fn map_part<F: FnOnce(K) -> K>(self, f: F) -> Self {
        CombinedPart(Affixed {
            combined: self.0.combined.map_part(f),
            ..self.0
        })
    }
}

impl<P, M, S, C> Contains<Affixed<P, M, S, C>> for Affixed<P, M, S, C> {
    fn map_part<F: FnOnce(Affixed<P, M, S, C>) -> Affixed<P, M, S, C>>(self, f: F) -> Self {
        f(self)
    }
}