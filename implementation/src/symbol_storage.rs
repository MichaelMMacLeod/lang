// use crate::{capacity::{self, Capacity}, graph::Graph};
// use std::{cell::Ref, collections::HashSet, rc::Rc, io::Read};

// pub struct SymbolStorage {
//     // 
//     buffer: Vec<u8>,
//     interner: HashSet<Rc<SymbolRef>>
// }

// type ReachableSymbols<'s> = &'s [SymbolRef];

// enum SymbolAccessError {}
// enum StorageRemovalError {}

// impl SymbolStorage {
//     fn read<R: Read>(&mut self, input: R) {

//     }

//     fn remove_unreachable<'s>(&mut self, r: ReachableSymbols<'s>) -> StorageRemovalError {
//         todo!()
//     }

//     fn trim(&mut self) {}

//     // fn get(&self, symbol: &Symbol) -> Result<&str, SymbolAccessError> {}

//     // fn with_capacity(capacity: Capacity) -> Self {
//     //     Self {
//     //         buffer: String::with_capacity(capacity.low),
//     //         capacity
//     //     }
//     // }

//     // fn read<R: Read>()
// }
