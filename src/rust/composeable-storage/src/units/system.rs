

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AtOrAbove<T>(T);

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AtOrBelow<T>(T);

#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct AtOrBetween<T: PartialOrd> {
    // invariant: at_or_above <= at_or_below
    at_or_above: T,
    at_or_below: T,
}
