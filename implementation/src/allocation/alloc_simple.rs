use std::{mem::size_of, marker::PhantomData};

use bitvec::prelude::BitArray;

pub struct SimpleAlloc<const B: usize, const F: usize, const NUM_BLOCKS: usize> {
    buffer: [u64; B],
    // A bit of 0 means the block is occupied, 1 means it is free
    free_blocks: BitArray<[u64; F]>,
}

// block_size x num_blocks
type Simple_64x64 = SimpleAlloc<64, 1, 64>;
type Simple_64x128 = SimpleAlloc<128, 2, 64>;

// type Simple1x4 = SimpleAlloc<8, {size_of::<usize>}>;

// macro_rules! simple_alloc {
//     ($block_size:expr, $num_blocks:expr) => {
//         impl SimpleAlloc<($block_size * $num_blocks) / 32, $num_blocks> {

//         }
//     };
// }

// simple_alloc!(32, 8);
// pub const fn simple_alloc_8_bit_blocks(const num_blocks: usize) {
//     const buffer_length: usize = num_blocks;
//     let free_blocks_length = num_blocks / 4;
//     SimpleAlloc::<buffer_length, free_blocks_length> {
//         buffer: [0; buffer_length],

//     }
// }

// impl<const NUM_BYTES: usize, const NUM_BLOCKS_MULT_USIZE: usize>
//     SimpleAlloc<NUM_BYTES, NUM_BLOCKS_MULT_USIZE>
// {
//     pub fn new() -> Self {
//         assert!(NUM_BYTES.is_power_of_two());
//         let num_blocks = NUM_BLOCKS_MULT_USIZE / size_of::<usize>();
//         Self {
//             buffer: [0; NUM_BYTES],
//             free_blocks: BitArray::ZERO,
//             num_blocks,
//         }
//     }
// }
