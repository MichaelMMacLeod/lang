pub trait TrySplitOnce: Sized {
    fn try_split_once(self, start_of_second: usize) -> Option<(Self, Self)>;
}

pub fn try_split_twice<B: TrySplitOnce>(
    b: B,
    start_of_second: usize,
    start_of_third: usize,
) -> Option<(B, B, B)> {
    let third_offset = start_of_third.checked_sub(start_of_second)?;
    let (prefix, rest) = b.try_split_once(start_of_second)?;
    let (middle, suffix) = rest.try_split_once(third_offset)?;
    Some((prefix, middle, suffix))
}
