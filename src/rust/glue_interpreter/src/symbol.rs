#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Symbol {
    data: String,
}

impl Symbol {
    pub fn new(data: String) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &String {
        &self.data
    }
}
