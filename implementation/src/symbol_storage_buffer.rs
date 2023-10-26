use std::{io::Read, str::Utf8Error};

use crate::symbol::Symbol;

pub struct StorableStr<'a> {
    str: &'a str,
}

impl<'a> TryFrom<&'a [u8]> for StorableStr<'a> {
    type Error = Utf8Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self {
            str: std::str::from_utf8(value)?,
        })
    }
}

#[derive(Clone, Copy)]
pub struct Key(usize);

pub struct BufferData<'a, const SIZE: usize> {
    pub data: &'a mut [u8; SIZE],
    pub key: Key,
}

pub trait Allocator {
    type AllocateFailure;
    fn allocate<const SIZE: usize>(&mut self) -> Result<BufferData<SIZE>, Self::AllocateFailure>;
}

pub trait Deallocator: Allocator {
    fn deallocate(&mut self, key: Key);
}

pub struct AlwaysFailsAllocator<F: Clone> {
    allocation_failure: F,
}

impl<F: Clone> Allocator for AlwaysFailsAllocator<F> {
    type AllocateFailure = F;

    fn allocate<const SIZE: usize>(&mut self) -> Result<BufferData<SIZE>, Self::AllocateFailure> {
        Err(self.allocation_failure.clone())
    }
}

pub struct FallbackAllocator<A1: Allocator, A2: Allocator> {
    primary: A1,
    fallback: A2,
}

pub struct SimpleSequentialAllocator {
    buffer: Vec<u8>,
}

pub struct SymbolRef {
    start: usize,
}

pub enum ReadSuccess {}
pub enum ReadError {}
pub enum RemoveError {}

impl SimpleSequentialAllocator {
    pub fn insert<R: Read>(
        &mut self,
        input: R,
        max_bytes_read: usize,
    ) -> Result<ReadSuccess, ReadError> {
        todo!()
    }

    pub fn remove(&mut self, s: SymbolRef) -> Option<RemoveError> {
        todo!()
    }

    pub fn compact(&mut self) {
        todo!()
    }
}
