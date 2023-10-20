use std::fmt::Formatter;

pub struct SubtermIndex(usize);

impl std::fmt::Debug for SubtermIndex {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self.0)
    }
}

impl From<usize> for SubtermIndex {
    fn from(u: usize) -> Self {
        Self(u)
    }
}
