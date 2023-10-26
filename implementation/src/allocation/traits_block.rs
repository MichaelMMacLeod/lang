pub trait Block: From<usize> + Into<usize> {}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct BasicBlock {
    key: usize,
    size: usize,
}

pub struct ExactSizeBlock<const SIZE: usize> {
    key: usize,
}

// impl Block {
//     pub fn new(key: usize, size: usize) -> Self {
//         Self {
//             key,
//             size,
//         }
//     }

//     pub fn key(&self) -> &usize {
//         &self.key
//     }

//     pub fn size(&self) -> &usize {
//         &self.size
//     }
// }