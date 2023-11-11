use super::contains::Contains;

pub trait ContainsRef<Part> {
    fn for_part<F: FnOnce(&Part)>(self, f: F);
}

impl<P, M: Contains<P>> ContainsRef<P> for M {
    fn for_part<F: FnOnce(&P)>(self, f: F) {
        self.map_part(|p| {
            f(&p);
            p
        });
    }
}