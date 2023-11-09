use std::{alloc::Layout, ptr::NonNull};

#[derive(Clone, Copy)]
pub struct Block {
    ptr: NonNull<[u8]>,

    // This is the layout that the Blk was allocated using. WARNING; DANGER:
    // this may be smaller than the length of 'ptr' given that allocate may
    // return a larger memory region than requested.
    blueprint: Option<Layout>,
}

impl Block {
    pub fn new(ptr: NonNull<[u8]>, blueprint: Option<Layout>) -> Self {
        Self { ptr, blueprint }
    }

    fn split<const N: usize>(&self, offsets: [usize; N]) -> Option<DisjointBlocks<N>> {
        todo!()
    }
}

impl Block {
    pub fn blueprint(&self) -> Option<Layout> {
        self.blueprint
    }
}

impl From<Block> for NonNull<[u8]> {
    fn from(value: Block) -> Self {
        value.ptr
    }
}

impl From<Block> for *mut [u8] {
    fn from(value: Block) -> Self {
        value.ptr.as_ptr()
    }
}

impl From<Block> for *mut u8 {
    fn from(value: Block) -> Self {
        let ptr: *mut [u8] = value.into();
        ptr as *mut u8
    }
}

impl From<Block> for NonNull<u8> {
    fn from(value: Block) -> Self {
        let ptr: *mut u8 = value.into();
        unsafe { NonNull::new_unchecked(ptr) }
    }
}

pub struct DisjointBlocks<const N: usize> {
    blocks: [Block; N],
}

impl<const N: usize> DisjointBlocks<N> {
    pub fn new(blocks: [Block; N]) -> Self {
        Self { blocks }
    }
}