use nom::{multi::many0, sequence::delimited, IResult};

use crate::{
    storage::{Storage, StorageKey},
    symbol::Symbol,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Compound {
    data: Vec<StorageKey>,
}

impl Compound {
    pub fn new(data: Vec<StorageKey>) -> Self {
        Self { data }
    }

    pub fn keys(&self) -> &[StorageKey] {
        &self.data
    }

    pub fn keys_mut(&mut self) -> &mut [StorageKey] {
        &mut self.data
    }
}
