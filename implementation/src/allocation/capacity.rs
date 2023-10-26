pub struct Capacity {
    min: usize,
    max: usize,
}

impl Capacity {
    pub fn new(min: usize, max: usize) -> Self {
        assert!(min <= max);
        Self { min, max }
    }

    pub fn min(&self) -> usize {
        self.min
    }

    pub fn max(&self) -> usize {
        self.max
    }
}
