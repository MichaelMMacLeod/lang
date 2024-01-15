use crate::{
    constructor::SingleConstructor,
    storage::{Storage, StorageKey},
};

pub enum Constructor2 {
    Copy(Vec<usize>),
    Symbol(String),
}

pub enum Index6 {
    ZeroPlus(usize),
    Middle {
        min_inclusive: usize,
        current: usize,
        max_inclusive: usize,
        current_compound_constructor_index: usize,
    },
}

pub struct Index7 {
    indices: Vec<Index6>,
}

pub struct ConstructorIterator {
    constructor: SingleConstructor,
    index: Index7,
}

impl ConstructorIterator {
    pub fn next(&mut self, storage: &Storage, key: StorageKey) -> Option<Constructor2> {
        todo!()
    }
}
