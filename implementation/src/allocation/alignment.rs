pub enum Alignment {
    Unaligned,
    AnyByteBoundary,
    TwoBytes,
    FourBytes,
    EightBytes,
}

pub struct Unaligned;

impl TryFrom<Alignment> for usize {
    type Error = Unaligned;
    fn try_from(value: Alignment) -> Result<Self, Self::Error> {
        match value {
            Alignment::Unaligned => Err(Unaligned),
            Alignment::AnyByteBoundary => Ok(1),
            Alignment::TwoBytes => Ok(2),
            Alignment::FourBytes => Ok(4),
            Alignment::EightBytes => Ok(8),
        }
    }
}