use nom::{
    bytes::complete::take_while,
    character::{is_alphabetic, is_alphanumeric},
    IResult,
};

use crate::storage::{Storage, Term};

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
