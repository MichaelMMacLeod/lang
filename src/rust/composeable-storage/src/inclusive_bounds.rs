#[derive(Clone, Copy)]
pub struct InclusiveLowerBound<O: PartialOrd>(O);

#[derive(Clone, Copy)]
pub struct InclusiveUpperBound<O: PartialOrd>(O);

#[derive(Clone, Copy)]
pub struct InclusiveBounds<O: PartialOrd> {
    lower: InclusiveLowerBound<O>,
    upper: InclusiveUpperBound<O>,
}

impl<O: PartialOrd> InclusiveBounds<O> {
    pub fn try_new(lower: InclusiveLowerBound<O>, upper: InclusiveUpperBound<O>) -> Option<Self> {
        lower.0.le(&upper.0).then(|| Self { lower, upper })
    }

    pub fn lower_ref(&self) -> &InclusiveLowerBound<O> {
        &self.lower
    }

    pub fn upper_ref(&self) -> &InclusiveUpperBound<O> {
        &self.upper
    }
}

impl<O: PartialOrd + Copy> InclusiveBounds<O> {
    pub fn lower(&self) -> InclusiveLowerBound<O> {
        self.lower
    }

    pub fn upper(&self) -> InclusiveUpperBound<O> {
        self.upper
    }
}