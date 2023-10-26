use bitvec::{prelude::BitArray, vec::BitVec};

use super::capacity::Capacity;

pub struct Stack<const MAX: usize> {
    buffer: BitArray<[usize; MAX]>,
}

pub struct Heap {
    buffer: BitVec,
    capacity: Capacity,
}

pub struct Memory<const STACK_MEM: usize> {
    stack: Stack<STACK_MEM>,
    heap: Heap,
}

impl<const STACK_MEM: usize> Memory<STACK_MEM> {
    pub fn new(heap: Capacity) -> Self {
        let mut v = BitVec::new();
        v.reserve_exact(heap.min());
        Self {
            stack: Stack {
                buffer: BitArray::ZERO,
            },
            heap: Heap {
                buffer: v,
                capacity: heap,
            },
        }
    }
}
