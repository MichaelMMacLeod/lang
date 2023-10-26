// use std::{io::Read, rc::Rc};

// use ahash::{HashMap, HashSet};
// use petgraph::csr::Csr;
// use slotmap::{new_key_type, SlotMap};

// use crate::{
//     capacity::Capacity,
//     symbol_storage::{SymbolRef, SymbolStorage},
// };

// new_key_type! { pub struct GraphKey; }

// pub struct Graph {
//     heap: Vec<GcNode>,
//     capacity: Capacity,
//     nodes: HashMap<>
//     symbols: SymbolStorage,
// }

// pub struct CompoundNodeRef {
//     start: usize,
//     len: usize,
// }

// pub struct GcNode {
//     reachable: bool,
//     node: Node,
// }

// pub enum Node {
//     Compound(CompoundNodeRef),
//     Singular(SymbolRef),
// }

// impl Graph {
//     pub fn read<R: Read>(&mut self, input: R) {
//         todo!()
//     }

    

//     // fn mark(&'graph mut self, )
// }
