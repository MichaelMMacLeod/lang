use super::block_dynamic::DynamicBlock;

pub struct Affixed<Pre, Mid, Suf, Com> {
    prefix: Pre,
    middle: Mid,
    suffix: Suf,
    combined: Com,
}

impl<Pre, Mid, Suf, Com> Affixed<Pre, Mid, Suf, Com> {
    pub fn new(prefix: Pre, middle: Mid, suffix: Suf, combined: Com) -> Self {
        Self {
            prefix,
            middle,
            suffix,
            combined,
        }
    }

    pub fn prefix(&self) -> &Pre {
        &self.prefix
    }

    pub fn middle(&self) -> &Mid {
        &self.middle
    }

    pub fn suffix(&self) -> &Suf {
        &self.suffix
    }

    pub fn combined(&self) -> &Com {
        &self.combined
    }

    pub fn map<Pre2, Mid2, Suf2, F: Fn(Pre, Mid, Suf) -> (Pre2, Mid2, Suf2)>(
        self,
        f: F,
    ) -> Affixed<Pre2, Mid2, Suf2, Com> {
        let (pre2, mid2, suf2) = f(self.prefix, self.middle, self.suffix);
        Affixed::new(pre2, mid2, suf2, self.combined)
    }

    pub fn map_combined<Com2, F: Fn(Com) -> Com2>(self, f: F) -> Affixed<Pre, Mid, Suf, Com2> {
        Affixed::new(self.prefix, self.middle, self.suffix, f(self.combined))
    }
}

impl<Pre, Mid, Suf> From<Affixed<Pre, Mid, Suf, DynamicBlock>> for DynamicBlock {
    fn from(value: Affixed<Pre, Mid, Suf, DynamicBlock>) -> Self {
        value.combined
    }
}