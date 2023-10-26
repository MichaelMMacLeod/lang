// use nom::Err;
// use slotmap::{SlotMap, new_key_type, Key};

// use crate::{symbol::Symbol, read::Token};

// pub fn parse<'i>(input: &'i [Token]) -> TokenTree<'i> {
//     TokenTree::from(input)
// }

// new_key_type! { struct TokenTreeKey; }

// struct TokenTree<'input_string> {
//     ancestry: Vec<TokenTreeKey>,
//     slotmap: SlotMap<TokenTreeKey, TokenNode<'input_string>>,
// }

// enum TokenNode<'input_string> {
//     Compound {
//         parent: Option<TokenTreeKey>,
//         children: &'input_string [TokenTreeKey],
//     },
//     Singular(Symbol<'input_string>),
// }

// // not exactly sure what the best values are here...
// static DEFAULT_SLOTMAP_CAPACITY: usize = 1024;
// static DEFAULT_ANCESTRY_CAPACITY: usize = 4 * DEFAULT_SLOTMAP_CAPACITY;

// struct NoTokensToParse;

// impl<'i> TryFrom<&'i [Token]> for TokenTree<'i> {
//     type Error = NoTokensToParse;

//     fn try_from(input: &'i [Token]) -> Result<Self, NoTokensToParse> {
//         if input.is_empty() {
//             Err(NoTokensToParse)
//         } else {
//             let mut slotmap = SlotMap::with_capacity_and_key(DEFAULT_SLOTMAP_CAPACITY);
//             let mut ancestry = Vec::with_capacity(DEFAULT_ANCESTRY_CAPACITY);
    
//             let mut number_of_opens = 0;
    
//             Ok(Self {
//                 slotmap,
//                 ancestry
//             })
//         }
//     }
// }

// impl Default for TokenTree {
//     fn default() -> Self {
//         Self { ancestry: Default::default(), slotmap: Default::default() }
//     }
// }