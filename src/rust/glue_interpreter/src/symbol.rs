use nom::{
    bytes::complete::take_while,
    character::{is_alphabetic, is_alphanumeric},
    IResult,
};

use crate::storage::{Storage, Term};

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Symbol {
    data: Vec<u8>,
}

impl Symbol {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
