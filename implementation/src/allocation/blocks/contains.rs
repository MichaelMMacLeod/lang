use super::split_at::TrySplitOnce;

pub trait Contains<Part> {
    fn map_part<F: FnOnce(Part) -> Part>(self, f: F) -> Self;
}