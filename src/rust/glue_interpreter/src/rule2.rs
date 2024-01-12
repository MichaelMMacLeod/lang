// use std::{
//     collections::{HashMap, HashSet},
//     sync::{Once, OnceLock},
// };

// use crate::{
//     compound::Compound,
//     parser::read,
//     storage::{Storage, StorageBFS, StorageKey, Term},
//     symbol::Symbol,
// };

// struct Env {
//     top: Node,
// }

// struct Node {
//     arrows: Vec<Arrow>,
//     constructor: Option<CompoundElement>,
// }

// struct Arrow {
//     predicates: PredicateSet,
//     target: Node,
// }

// enum SingleConstructor {
//     Copy(Vec<Index>),
//     Symbol(String),
//     Compound(Vec<CompoundElement>),
// }

// struct CompoundElement {
//     single_constructor: SingleConstructor,
//     dot_dot_count: usize,
// }

// // enum Constructor {
// //     Copy(Vec<Index>),
// //     Symbol(String),
// //     Compound(Vec<CompoundElem>),
// // }

// // struct MiddleConstructor {
// //     constructor: Constructor,
// //     // MUST start with zero or more non-middle indices
// //     // followed by exactly one middle index.
// //     shared_indices: Vec<Index>,
// // }

// // struct CompoundElem {
// //     constructor: Constructor,
// //     dot_dot_count: usize,
// // }

// // struct CompoundConstructor {
// //     start: Vec<Constructor>,
// //     middle: Option<Box<MiddleConstructor>>,
// //     end: Vec<Constructor>,
// // }


// fn get_with_offsets(
//     storage: &Storage,
//     mut k: StorageKey,
//     indices: &[Index],
//     offsets: &[usize],
// ) -> StorageKey {
//     let mut offsets = offsets.iter().copied();
//     for index in indices {
//         match storage.get(k).unwrap() {
//             Term::Compound(c) => {
//                 let zp = match index {
//                     Index::ZeroPlus(zp) => *zp,
//                     Index::LengthMinus(lm) => c.keys().len() - lm,
//                     Index::Middle(m) => {
//                         let offset = offsets.next().expect("no next offset");
//                         m.starting_at_zero_plus() + offset
//                     }
//                 };
//                 k = c.keys()[zp];
//             }
//             _ => panic!("attempt to index into non-compound term"),
//         }
//     }
//     assert!(offsets.next().is_none());
//     k
// }

// fn construct_single(
//     storage: &mut Storage,
//     constructor: &Constructor,
//     source: StorageKey,
//     destination: StorageKey,
//     offsets: &[usize],
// ) {
//     match constructor {
//         Constructor::Copy(indices) => {
//             let new_term = get_with_offsets(storage, source, indices, offsets);
//             let new_term = storage.get(new_term).unwrap().clone();
//             storage.replace(destination, new_term);
//         }
//         Constructor::Symbol(string) => {
//             storage.replace(destination, Term::Symbol(Symbol::new(string.clone())));
//         }
//         Constructor::Compound(compound) => {
//             // let start_part: Vec<StorageKey> = compound
//             //     .start
//             //     .iter()
//             //     .map(|constructor| {
//             //         let destination = storage.insert(Term::Symbol(Symbol::new("".into())));
//             //         construct_single(storage, constructor, source, destination, offsets);
//             //         destination
//             //     })
//             //     .collect();
//             // let end_part: Vec<StorageKey> = compound
//             //     .end
//             //     .iter()
//             //     .map(|constructor| {
//             //         let destination = storage.insert(Term::Symbol(Symbol::new("".into())));
//             //         construct_single(storage, constructor, source, destination, offsets);
//             //         destination
//             //     })
//             //     .collect();
//             // let middle_part: Vec<StorageKey> = match &compound.middle {
//             //     Some(middle) => {
//             //         let repetitions = count_repetitions(storage, source, &middle.shared_indices);
//             //         let mut result = Vec::with_capacity(repetitions);
//             //         for offset in 0..repetitions {
//             //             // let offsets: Vec<usize> =
//             //             //     [offset].iter().chain(offsets.iter()).copied().collect();
//             //             let offsets: Vec<usize> =
//             //                 offsets.iter().chain([offset].iter()).copied().collect();
//             //             // [offset].iter().chain(offsets.iter()).copied().collect();
//             //             let destination = storage.insert(Term::Symbol(Symbol::new("".into())));
//             //             construct_single(
//             //                 storage,
//             //                 &middle.constructor,
//             //                 source,
//             //                 destination,
//             //                 &offsets,
//             //             );
//             //             result.push(destination)
//             //         }
//             //         result
//             //     }
//             //     None => vec![],
//             // };
//             // let combined: Vec<StorageKey> = start_part
//             //     .into_iter()
//             //     .chain(middle_part.into_iter())
//             //     .chain(end_part.into_iter())
//             //     .collect();

//             // storage.replace(destination, Term::Compound(Compound::new(combined)));
//             todo!()
//         }
//     }
// }

// struct Rule2 {
//     predicate: PredicateSet,
//     constructor: Constructor,
// }

// // (for (<var> : symbol) .. <predicate> -> <constructor>)
// // const fixpoint_rule_predicate: PredicateList = PredicateList { list: vec![] };

// struct VarMap {
//     map: HashMap<String, Option<Vec<Index>>>,
// }

// fn compile_rule2(storage: &Storage, k: StorageKey) -> Option<Rule2> {
//     static RULE_PREDICATE: OnceLock<PredicateSet> = OnceLock::new();
//     let rule_predicate = RULE_PREDICATE.get_or_init(|| PredicateSet {
//         set: [
//             IndexedPredicate {
//                 indices: vec![],
//                 predicate: Predicate::LengthGreaterThanOrEqualTo(4),
//             },
//             IndexedPredicate {
//                 indices: vec![Index::ZeroPlus(0)],
//                 predicate: Predicate::SymbolEqualTo("for".into()),
//             },
//             IndexedPredicate {
//                 indices: vec![Index::LengthMinus(2)],
//                 predicate: Predicate::SymbolEqualTo("->".into()),
//             },
//         ]
//         .into_iter()
//         .collect(),
//     });

//     if !matches(storage, k, rule_predicate) {
//         return None;
//     }

//     let c = storage.get_compound(k).unwrap().keys();
//     let length = c.len();
//     let predicate = c[length - 3];
//     let constructor = c[length - 1];
//     let mut variables = VarMap {
//         map: HashMap::new(),
//     };
//     for &k in &c[1..=(length - 4)] {
//         if let Some(s) = storage.get_symbol(k) {
//             variables.map.insert(s.data().clone(), None);
//         } else {
//             return None;
//         }
//     }

//     let Some(predicate_set) = compile_rule_predicate(storage, &mut variables, k) else {
//         return None;
//     };

//     todo!()
// }

// fn compile_rule_constructor(
//     storage: &mut Storage,
//     variables: &mut VarMap,
//     k: StorageKey,
// ) -> Option<Constructor> {
//     match storage.get(k).unwrap() {
//         Term::Symbol(s) => {
//             if let Some(indices) = variables.map.get(s.data()) {
//                 let indices = indices.clone().unwrap();
//                 Some(Constructor::Copy(indices))
//             } else {
//                 Some(Constructor::Symbol(s.data().clone()))
//             }
//         }
//         Term::Compound(c) => {
//             let mut keys = c.keys().iter().rev().cloned();

//             let mut start: Vec<Constructor> = vec![];
//             let mut middle: Option<MiddleConstructor> = None;
//             let mut end: Vec<Constructor> = vec![];

//             while let Some(key) = keys.next() {
//                 // if

//                 // ((point (x ..) (y ..)) ..) -> (vals x .. .. y .. ..)
//             }

//             todo!()
//         }
//         _ => None,
//     }
// }

// fn compile_rule_predicate(
//     storage: &Storage,
//     variables: &mut VarMap,
//     k: StorageKey,
// ) -> Option<PredicateSet> {
//     let mut predicate_set = PredicateSet::new();
//     compile_rule_predicate_list(storage, variables, k, &mut predicate_set, &[])
//         .map(|_| predicate_set)
// }

// enum RuleCompilationError {
//     VarUsedMoreThanOnce(String),
// }

// fn compile_rule_predicate_list(
//     storage: &Storage,
//     variables: &mut VarMap,
//     k: StorageKey,
//     list: &mut PredicateSet,
//     indices: &[Index],
// ) -> Option<()> {
//     match storage.get(k).unwrap() {
//         Term::Symbol(s) => {
//             if variables.map.contains_key(s.data()) {
//                 if variables.map.get(s.data()).is_some() {
//                     None // var used more than once is an error
//                 } else {
//                     let _ = variables
//                         .map
//                         .get_mut(s.data())
//                         .unwrap()
//                         .insert(indices.to_vec());
//                     Some(())
//                 }
//             } else {
//                 list.set.insert(IndexedPredicate {
//                     indices: indices.to_vec(),
//                     predicate: Predicate::SymbolEqualTo(s.data().clone()),
//                 });
//                 Some(())
//             }
//         }
//         Term::Compound(c) => {
//             fn is_dot_dotted(storage: &Storage, keys: &[StorageKey], index: usize) -> bool {
//                 let Some(index2) = index.checked_add(1) else {
//                     return false;
//                 };
//                 let k1 = keys.get(index);
//                 let k2 = keys.get(index2);
//                 match (k1, k2) {
//                     (Some(_), Some(t2)) => {
//                         let Some(s) = storage.get_symbol(*t2) else {
//                             return false;
//                         };
//                         s.data() == ".."
//                     }
//                     _ => false,
//                 }
//             }
//             let mut before_dot_dot = true;
//             let mut at_dot_dot = false;
//             for (index, &k) in c.keys().iter().enumerate() {
//                 if at_dot_dot {
//                     at_dot_dot = false;
//                     continue;
//                 }
//                 if is_dot_dotted(storage, c.keys(), index) {
//                     if before_dot_dot {
//                         before_dot_dot = false;
//                         at_dot_dot = true;
//                         let indices: Vec<Index> = indices
//                             .iter()
//                             .chain(&[Index::Middle(MiddleIndices {
//                                 starting_at_zero_plus(): index,
//                                 // because we are inside the loop we know length >= 1 and index < length
//                                 // len - index = number of elements after the dot-dotted term (including the "..")
//                                 // len - index - 1 = number of elements after the dot-dotted term (excluding the "..")
//                                 // [ 0 1 2 .. 3 4 5 6 ]
//                                 // start: 2
//                                 // end: length - 5
//                                 ending_at_length_minus(): c.keys().len() - index - 1,
//                             })])
//                             .cloned()
//                             .collect();
//                         compile_rule_predicate_list(storage, variables, k, list, &indices);
//                     } else {
//                         // Illegal to have more than one dot-dotted term per compound term
//                         return None;
//                     }
//                 } else if before_dot_dot {
//                     let indices: Vec<Index> = indices
//                         .iter()
//                         .chain(&[Index::ZeroPlus(index)])
//                         .cloned()
//                         .collect();
//                     compile_rule_predicate_list(storage, variables, k, list, &indices);
//                 } else {
//                     let indices: Vec<Index> = indices
//                         .iter()
//                         .chain(&[Index::LengthMinus(
//                             c.keys().len().checked_sub(index).unwrap(), // index.checked_sub(1).unwrap(), /* -1 to ignore the ".." */
//                         )])
//                         .cloned()
//                         .collect();
//                     compile_rule_predicate_list(storage, variables, k, list, &indices);
//                 }
//             }
//             list.set.insert(IndexedPredicate {
//                 indices: indices.to_vec(),
//                 predicate: if before_dot_dot {
//                     Predicate::LengthEqualTo(c.keys().len())
//                 } else {
//                     Predicate::LengthGreaterThanOrEqualTo(
//                         c.keys().len().checked_sub(2).unwrap(), /* -2 to ignore the "<T> .." */
//                     )
//                 },
//             });
//             Some(())
//         }
//         _ => None,
//     }
// }

// mod test {
//     use super::*;

//     #[test]
//     fn c1() {
//         let mut storage = Storage::new();
//         // let k = read(&mut storage, "(f ((g x) ..))").unwrap();
//         let k = read(&mut storage, "(zp0 zp1 zp2 (x 1 2 3) .. lm3 lm2 lm1)").unwrap();
//         let mut variables = VarMap {
//             map: [("x".into(), None)].into_iter().collect(),
//         };
//         let predicate = compile_rule_predicate(&storage, &mut variables, k).unwrap();
//         let expected = PredicateSet {
//             set: [
//                 IndexedPredicate {
//                     indices: vec![],
//                     predicate: Predicate::LengthGreaterThanOrEqualTo(6),
//                 },
//                 IndexedPredicate {
//                     indices: vec![Index::Middle(MiddleIndices {
//                         starting_at_zero_plus(): 3,
//                         ending_at_length_minus(): 4,
//                     })],
//                     predicate: Predicate::LengthEqualTo(4),
//                 },
//                 IndexedPredicate {
//                     indices: vec![
//                         Index::Middle(MiddleIndices {
//                             starting_at_zero_plus(): 3,
//                             ending_at_length_minus(): 4,
//                         }),
//                         Index::ZeroPlus(1),
//                     ],
//                     predicate: Predicate::SymbolEqualTo("1".into()),
//                 },
//                 IndexedPredicate {
//                     indices: vec![
//                         Index::Middle(MiddleIndices {
//                             starting_at_zero_plus(): 3,
//                             ending_at_length_minus(): 4,
//                         }),
//                         Index::ZeroPlus(2),
//                     ],
//                     predicate: Predicate::SymbolEqualTo("2".into()),
//                 },
//                 IndexedPredicate {
//                     indices: vec![
//                         Index::Middle(MiddleIndices {
//                             starting_at_zero_plus(): 3,
//                             ending_at_length_minus(): 4,
//                         }),
//                         Index::ZeroPlus(3),
//                     ],
//                     predicate: Predicate::SymbolEqualTo("3".into()),
//                 },
//                 IndexedPredicate {
//                     indices: vec![Index::ZeroPlus(0)],
//                     predicate: Predicate::SymbolEqualTo("zp0".into()),
//                 },
//                 IndexedPredicate {
//                     indices: vec![Index::ZeroPlus(1)],
//                     predicate: Predicate::SymbolEqualTo("zp1".into()),
//                 },
//                 IndexedPredicate {
//                     indices: vec![Index::ZeroPlus(2)],
//                     predicate: Predicate::SymbolEqualTo("zp2".into()),
//                 },
//                 IndexedPredicate {
//                     indices: vec![Index::LengthMinus(3)],
//                     predicate: Predicate::SymbolEqualTo("lm3".into()),
//                 },
//                 IndexedPredicate {
//                     indices: vec![Index::LengthMinus(2)],
//                     predicate: Predicate::SymbolEqualTo("lm2".into()),
//                 },
//                 IndexedPredicate {
//                     indices: vec![Index::LengthMinus(1)],
//                     predicate: Predicate::SymbolEqualTo("lm1".into()),
//                 },
//             ]
//             .into(),
//         };
//         assert_eq!(predicate, expected);
//     }

//     #[test]
//     fn t1() {
//         // (for x (x ..) -> ((a x ..) b (x c) ..))
//         let result_constructor3 = Constructor::Compound(vec![CompoundElem {
//             constructor: Constructor::Compound(vec![
//                 CompoundElem {
//                     constructor: Constructor::Symbol("a".into()),
//                     dot_dot_count: 0,
//                 },
//                 CompoundElem {
//                     constructor: Constructor::Copy(vec![Index::Middle(MiddleIndices {
//                         starting_at_zero_plus(): 0,
//                         ending_at_length_minus(): 1,
//                     })]),
//                     dot_dot_count: 1,
//                 },
//             ]),
//             dot_dot_count: 0,
//         }]);
//         // let result_constructor3 = Constructor::Compound(vec![CompoundElem::NotDotDotted(
//         //     Constructor::Compound(vec![
//         //         CompoundElem::NotDotDotted(Constructor::Symbol("a".into())),
//         //         CompoundElem::DotDotted(Constructor::Copy(vec![Index::Middle(
//         //             MiddleIndices {
//         //                 starting_at_zero_plus(): 0,
//         //                 ending_at_length_minus(): 1,
//         //             },
//         //         )])),
//         //     ]),
//         // ),
//         // CompoundElem::NotDotDotted(Constructor::Symbol("b".into())),
//         // CompoundElem::DotDotted(Constructor::Compound(vec![

//         // ]))]);
//         // let result_constructor3 = Constructor::Compound(CompoundConstructor {
//         //     start: vec![
//         //         Constructor::Compound(CompoundConstructor {
//         //             start: vec![Constructor::Symbol("a".into())],
//         //             middle: Some(Box::new(MiddleConstructor {
//         //                 constructor: Constructor::Copy(vec![Index::Middle(MiddleIndices {
//         //                     starting_at_zero_plus(): 0,
//         //                     ending_at_length_minus(): 1,
//         //                 })]),
//         //                 shared_indices: vec![Index::Middle(MiddleIndices {
//         //                     starting_at_zero_plus(): 0,
//         //                     ending_at_length_minus(): 1,
//         //                 })],
//         //             })),
//         //             end: vec![],
//         //         }),
//         //         Constructor::Symbol("b".into()),
//         //     ],
//         //     middle: Some(Box::new(MiddleConstructor {
//         //         constructor: Constructor::Compound(CompoundConstructor {
//         //             start: vec![
//         //                 Constructor::Copy(vec![Index::Middle(MiddleIndices {
//         //                     starting_at_zero_plus(): 0,
//         //                     ending_at_length_minus(): 1,
//         //                 })]),
//         //                 Constructor::Symbol("c".into()),
//         //             ],
//         //             middle: None,
//         //             end: vec![],
//         //         }),
//         //         shared_indices: vec![Index::Middle(MiddleIndices {
//         //             starting_at_zero_plus(): 0,
//         //             ending_at_length_minus(): 1,
//         //         })],
//         //     })),
//         //     end: vec![],
//         // });
//         let mut storage = Storage::new();
//         let source = read(&mut storage, "(1 2 3 4 5 6 7 8 9)").unwrap();
//         let destination = read(&mut storage, "()").unwrap();
//         construct_single(&mut storage, &result_constructor3, source, destination, &[]);
//         storage.println(destination, false);
//     }

//     #[test]
//     fn t2() {
//         // (for x ((g x) ..) -> ((a x ..) b (x c) ..))
//         let result_constructor3 = Constructor::Compound(vec![]);
//         // let result_constructor3 = Constructor::Compound(CompoundConstructor {
//         //     start: vec![
//         //         Constructor::Compound(CompoundConstructor {
//         //             start: vec![Constructor::Symbol("a".into())],
//         //             middle: Some(Box::new(MiddleConstructor {
//         //                 constructor: Constructor::Copy(vec![
//         //                     Index::Middle(MiddleIndices {
//         //                         starting_at_zero_plus(): 0,
//         //                         ending_at_length_minus(): 1,
//         //                     }),
//         //                     Index::ZeroPlus(1),
//         //                 ]),
//         //                 shared_indices: vec![Index::Middle(MiddleIndices {
//         //                     starting_at_zero_plus(): 0,
//         //                     ending_at_length_minus(): 1,
//         //                 })],
//         //             })),
//         //             end: vec![],
//         //         }),
//         //         Constructor::Symbol("b".into()),
//         //     ],
//         //     middle: Some(Box::new(MiddleConstructor {
//         //         constructor: Constructor::Compound(CompoundConstructor {
//         //             start: vec![
//         //                 Constructor::Copy(vec![
//         //                     Index::Middle(MiddleIndices {
//         //                         starting_at_zero_plus(): 0,
//         //                         ending_at_length_minus(): 1,
//         //                     }),
//         //                     Index::ZeroPlus(1),
//         //                 ]),
//         //                 Constructor::Symbol("c".into()),
//         //             ],
//         //             middle: None,
//         //             end: vec![],
//         //         }),
//         //         shared_indices: vec![Index::Middle(MiddleIndices {
//         //             starting_at_zero_plus(): 0,
//         //             ending_at_length_minus(): 1,
//         //         })],
//         //     })),
//         //     end: vec![],
//         // });
//         let mut storage = Storage::new();
//         let source = read(&mut storage, "((g 1) (g 2) (g 3) (g 4) (g 5))").unwrap();
//         let destination = read(&mut storage, "()").unwrap();
//         construct_single(&mut storage, &result_constructor3, source, destination, &[]);
//         storage.println(destination, false);
//     }

//     #[test]
//     fn t3() {
//         // (for x (f ((g x) ..)) -> ((a x ..) b (x c) ..))
//         let predicates = PredicateSet {
//             set: [
//                 IndexedPredicate {
//                     indices: vec![],
//                     predicate: Predicate::LengthEqualTo(2),
//                 },
//                 IndexedPredicate {
//                     indices: vec![Index::ZeroPlus(0)],
//                     predicate: Predicate::SymbolEqualTo("f".into()),
//                 },
//                 IndexedPredicate {
//                     indices: vec![Index::ZeroPlus(1)],
//                     predicate: Predicate::LengthGreaterThanOrEqualTo(0),
//                 },
//                 IndexedPredicate {
//                     indices: vec![
//                         Index::ZeroPlus(1),
//                         Index::Middle(MiddleIndices {
//                             starting_at_zero_plus(): 0,
//                             ending_at_length_minus(): 1,
//                         }),
//                     ],
//                     predicate: Predicate::LengthEqualTo(2),
//                 },
//                 IndexedPredicate {
//                     indices: vec![
//                         Index::ZeroPlus(1),
//                         Index::Middle(MiddleIndices {
//                             starting_at_zero_plus(): 0,
//                             ending_at_length_minus(): 1,
//                         }),
//                         Index::ZeroPlus(0),
//                     ],
//                     predicate: Predicate::SymbolEqualTo("g".into()),
//                 },
//             ]
//             .into_iter()
//             .collect(),
//         };
//         let result_constructor3 = Constructor::Compound(vec![]);
//         // let result_constructor3 = Constructor::Compound(CompoundConstructor {
//         //     start: vec![
//         //         Constructor::Compound(CompoundConstructor {
//         //             start: vec![Constructor::Symbol("a".into())],
//         //             middle: Some(Box::new(MiddleConstructor {
//         //                 constructor: Constructor::Copy(vec![
//         //                     Index::ZeroPlus(1),
//         //                     Index::Middle(MiddleIndices {
//         //                         starting_at_zero_plus(): 0,
//         //                         ending_at_length_minus(): 1,
//         //                     }),
//         //                     Index::ZeroPlus(1),
//         //                 ]),
//         //                 shared_indices: vec![
//         //                     Index::ZeroPlus(1),
//         //                     Index::Middle(MiddleIndices {
//         //                         starting_at_zero_plus(): 0,
//         //                         ending_at_length_minus(): 1,
//         //                     }),
//         //                 ],
//         //             })),
//         //             end: vec![],
//         //         }),
//         //         Constructor::Symbol("b".into()),
//         //     ],
//         //     middle: Some(Box::new(MiddleConstructor {
//         //         constructor: Constructor::Compound(CompoundConstructor {
//         //             start: vec![
//         //                 Constructor::Copy(vec![
//         //                     Index::ZeroPlus(1),
//         //                     Index::Middle(MiddleIndices {
//         //                         starting_at_zero_plus(): 0,
//         //                         ending_at_length_minus(): 1,
//         //                     }),
//         //                     Index::ZeroPlus(1),
//         //                 ]),
//         //                 Constructor::Symbol("c".into()),
//         //             ],
//         //             middle: None,
//         //             end: vec![],
//         //         }),
//         //         shared_indices: vec![
//         //             Index::ZeroPlus(1),
//         //             Index::Middle(MiddleIndices {
//         //                 starting_at_zero_plus(): 0,
//         //                 ending_at_length_minus(): 1,
//         //             }),
//         //         ],
//         //     })),
//         //     end: vec![],
//         // });
//         let mut storage = Storage::new();
//         let source = read(&mut storage, "(f ((g 1) (g 2) (g 2) (g 3) (g 4) (g 5)))").unwrap();
//         assert!(matches(&storage, source, &predicates));
//         let destination = read(&mut storage, "()").unwrap();
//         construct_single(&mut storage, &result_constructor3, source, destination, &[]);
//         storage.println(destination, false);
//     }

//     #[test]
//     fn t4() {
//         // (for x ((x ..) ..) -> ((x ..) ..)
//         let result_constructor3 = Constructor::Compound(vec![]);
//         // let result_constructor3 = Constructor::Compound(CompoundConstructor {
//         //     start: vec![],
//         //     middle: Some(Box::new(MiddleConstructor {
//         //         constructor: Constructor::Compound(CompoundConstructor {
//         //             start: vec![],
//         //             middle: Some(Box::new(MiddleConstructor {
//         //                 constructor: Constructor::Copy(vec![
//         //                     Index::Middle(MiddleIndices {
//         //                         starting_at_zero_plus(): 0,
//         //                         ending_at_length_minus(): 1,
//         //                     }),
//         //                     Index::Middle(MiddleIndices {
//         //                         starting_at_zero_plus(): 0,
//         //                         ending_at_length_minus(): 1,
//         //                     }),
//         //                 ]),
//         //                 shared_indices: vec![
//         //                     Index::Middle(MiddleIndices {
//         //                         starting_at_zero_plus(): 0,
//         //                         ending_at_length_minus(): 1,
//         //                     }),
//         //                     Index::Middle(MiddleIndices {
//         //                         starting_at_zero_plus(): 0,
//         //                         ending_at_length_minus(): 1,
//         //                     }),
//         //                 ],
//         //             })),
//         //             end: vec![],
//         //         }),
//         //         shared_indices: vec![
//         //             Index::Middle(MiddleIndices {
//         //                 starting_at_zero_plus(): 0,
//         //                 ending_at_length_minus(): 1,
//         //             }),
//         //             Index::Middle(MiddleIndices {
//         //                 starting_at_zero_plus(): 0,
//         //                 ending_at_length_minus(): 1,
//         //             }),
//         //         ],
//         //     })),
//         //     end: vec![],
//         // });
//         let mut storage = Storage::new();
//         let source = read(&mut storage, "((1 2 3 4))").unwrap();
//         // assert!(matches(&storage, source, &predicates));
//         let destination = read(&mut storage, "()").unwrap();
//         construct_single(&mut storage, &result_constructor3, source, destination, &[]);
//         storage.println(destination, false);
//     }
// }

// // (for x (a (b x .. c) .. d) -> (e x .. .. f))
// // let pattern_predicate = PredicateList {
// //     list: vec![
// //         IndexedPredicate {
// //             indices: vec![],
// //             predicate: Predicate::LengthGreaterThanOrEqualTo(2),
// //         },
// //         IndexedPredicate {
// //             indices: vec![Index::ZeroPlus(0)],
// //             predicate: Predicate::SymbolEqualTo("a".into()),
// //         },
// //         IndexedPredicate {
// //             indices: vec![Index::LengthMinus(1)],
// //             predicate: Predicate::SymbolEqualTo("d".into()),
// //         },
// //         IndexedPredicate {
// //             indices: vec![Index::Middle(MiddleIndices {
// //                 starting_at_zero_plus(): 1,
// //                 ending_at_length_minus(): 2,
// //             })],
// //             predicate: Predicate::LengthGreaterThanOrEqualTo(2),
// //         },
// //         IndexedPredicate {
// //             indices: vec![
// //                 Index::Middle(MiddleIndices {
// //                     starting_at_zero_plus(): 1,
// //                     ending_at_length_minus(): 2,
// //                 }),
// //                 Index::ZeroPlus(0),
// //             ],
// //             predicate: Predicate::SymbolEqualTo("b".into()),
// //         },
// //         IndexedPredicate {
// //             indices: vec![
// //                 Index::Middle(MiddleIndices {
// //                     starting_at_zero_plus(): 1,
// //                     ending_at_length_minus(): 2,
// //                 }),
// //                 Index::LengthMinus(1),
// //             ],
// //             predicate: Predicate::SymbolEqualTo("c".into()),
// //         },
// //     ],
// // };
// // let result_constructor = ConstructorList {
// //     list: vec![
// //         IndexedConstructor {
// //             indices: vec![IndexedConstructorIndices::ZeroPlus(0)],
// //             constructor: Constructor::Symbol("e".into()),
// //         },
// //         IndexedConstructor {
// //             indices: vec![IndexedConstructorIndices::LengthMinus(1)],
// //             constructor: Constructor::Symbol("f".into()),
// //         },
// //         IndexedConstructor {
// //             indices: vec![IndexedConstructorIndices::Middle],
// //             constructor: Constructor::Copy(vec![
// //                 Index::Middle(MiddleIndices {
// //                     starting_at_zero_plus(): 1,
// //                     ending_at_length_minus(): 2,
// //                 }),
// //                 Index::Middle(MiddleIndices {
// //                     starting_at_zero_plus(): 1,
// //                     ending_at_length_minus(): 2,
// //                 }),
// //             ]),
// //         },
// //     ],
// // };
// // // let result_constructor2 = Constructor2::Compound(CompoundConstructor2 {
// // //     start: vec![Constructor2::Symbol("e".into())],
// // //     middle: Some(Box::new(Constructor2::Copy(vec![
// // //         Index::Middle(MiddleIndices {
// // //             starting_at_zero_plus(): 1,
// // //             ending_at_length_minus(): 2,
// // //         }),
// // //         Index::Middle(MiddleIndices {
// // //             starting_at_zero_plus(): 1,
// // //             ending_at_length_minus(): 2,
// // //         }),
// // //     ]))),
// // //     end: vec![Constructor2::Symbol("f".into())],
// // // });

// // struct ActiveConstructionIndex {
// //     zero_plus: usize,
// // }

// // struct ActiveConstructionIndicesList {
// //     list: Vec<ActiveConstructionIndex>,
// // }

// // fn construct_single(
// //     storage: &mut Storage,
// //     constructor: &Constructor2,
// //     k: StorageKey,
// // ) {
// //     match constructors.list.first() {
// //         Some(ic) => match &ic.constructor {
// //             Constructor::Symbol(s) => {
// //                 let s = storage.insert(Term::Symbol(Symbol::new(s.clone())));
// //             }
// //             Constructor::Copy(_) => todo!(),
// //         },
// //         None => {}
// //     }
// // }

// // use std::{
// //     cmp::{Ordering, Reverse},
// //     collections::BinaryHeap,
// // };

// // use crate::{
// //     rule::{MultiPattern, Rule, SinglePattern},
// //     symbol::Symbol,
// // };

// // // pub enum EqualityCheck {
// // //     Length(usize),
// // //     Symbol(String),
// // //     SubtermLocation(SubtermLocation, Box<EqualityCheck>),
// // // }

// // // pub struct SubtermLocation {
// // //     indices: Vec<usize>,
// // // }

// // // pub enum Assignment {
// // //     Length(usize),
// // //     Symbol(Symbol),
// // //     SubtermLocation(SubtermLocation, Box<Assignment>),
// // // }

// // // pub enum Arrow {
// // //     Nonfixed(NonfixedArrow),
// // //     Fixpoint(FixpointArrow),
// // // }

// // // pub struct NonfixedArrow {
// // //     checks: Vec<EqualityCheck>,
// // //     destination: Box<Node>,
// // // }

// // // pub struct FixpointArrow {
// // //     checks: Vec<EqualityCheck>,
// // // }

// // // pub struct Node {
// // //     assignments: Vec<Assignment>,
// // //     arrows: Vec<Arrow>,
// // // }

// // // pub struct Env {
// // //     arrows: Vec<Arrow>,
// // // }

// // #[derive(Clone)]
// // pub struct ConstraintList {
// //     list: Vec<IndexedConstraint>,
// // }

// // #[derive(Clone)]
// // pub struct IndexedConstraint {
// //     indices: Vec<Index>,
// //     constraint: Constraint,
// // }

// // #[derive(Clone)]
// // pub enum Index {
// //     FromStart(usize),
// //     LengthMinus(usize),
// //     DotDot(DotDotIndex),
// // }

// // #[derive(Clone)]
// // pub struct DotDotIndex {
// //     greater_than: usize,
// //     less_than_length_minus: usize,
// // }

// // #[derive(Clone, PartialEq, Eq, Debug)]
// // pub enum Constraint {
// //     SymbolEqualTo(String),
// //     LengthEqualTo(usize),
// //     LengthGreaterThanOrEqualTo(usize),
// // }

// // impl PartialOrd for Constraint {
// //     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
// //         (self.eq(other))
// //             .then(|| Ordering::Equal)
// //             .or_else(|| match (&self, &other) {
// //                 (Constraint::LengthEqualTo(le), Constraint::LengthGreaterThanOrEqualTo(lge)) => {
// //                     Some(lge.cmp(le))
// //                 }
// //                 (Constraint::LengthGreaterThanOrEqualTo(lge), Constraint::LengthEqualTo(le)) => {
// //                     Some(lge.cmp(le))
// //                 }
// //                 (
// //                     Constraint::LengthGreaterThanOrEqualTo(lge1),
// //                     Constraint::LengthGreaterThanOrEqualTo(lge2),
// //                 ) => Some(lge2.cmp(lge1)),
// //                 _ => None,
// //             })
// //     }
// // }

// // pub fn partition_count(c: &Constraint, constraints: &[Constraint]) -> usize {
// //     constraints
// //         .into_iter()
// //         .filter(|c2| c2 <= &c)
// //         .count()
// //         .abs_diff(constraints.len())
// // }

// // // pub fn count_encompassing_constraints<'a, C: IntoIterator<Item = &'a Constraint>>(
// // //     c: &Constraint,
// // //     constraints: C,
// // // ) -> usize {
// // //     constraints.into_iter().filter(|c2| c <= c2).count()
// // // }

// // // pub fn partition_count<'a, C: IntoIterator<Item = &'a Constraint>>(
// // //     c: &Constraint,
// // //     constraints: C,
// // // ) -> usize {
// // //     constraints.into_iter().filter(|c2| c <= c2).count()
// // // }

// // pub fn best_partitioning_constraint(constraints: &[Constraint]) -> Option<Constraint> {
// //     struct PartitionCount {
// //         constraint: Constraint,
// //         count: usize,
// //     }
// //     impl PartialEq for PartitionCount {
// //         fn eq(&self, other: &Self) -> bool {
// //             self.count == other.count
// //         }
// //     }
// //     impl Eq for PartitionCount {}
// //     impl PartialOrd for PartitionCount {
// //         fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
// //             self.count.partial_cmp(&other.count)
// //         }
// //     }
// //     impl Ord for PartitionCount {
// //         fn cmp(&self, other: &Self) -> Ordering {
// //             self.count.cmp(&other.count)
// //         }
// //     }
// //     constraints
// //         .into_iter()
// //         .map(|c| {
// //             Reverse(PartitionCount {
// //                 constraint: c.clone(),
// //                 count: partition_count(c, constraints),
// //             })
// //         })
// //         .collect::<BinaryHeap<_>>()
// //         .pop()
// //         .map(|c| c.0.constraint)
// // }

// // #[cfg(test)]
// // mod test {
// //     use super::*;
// //     use Constraint::*;

// //     #[test]
// //     fn partition_count0() {
// //         let cs = [
// //             LengthEqualTo(10),
// //             LengthEqualTo(0),
// //             LengthEqualTo(1),
// //             LengthEqualTo(10),
// //             LengthEqualTo(2),
// //         ];
// //         assert!(LengthEqualTo(10) <= LengthEqualTo(10));
// //         assert!(LengthEqualTo(10) <= LengthEqualTo(10));
// //         // assert_eq!(partition_count(&LengthEqualTo(10), &cs), 1);
// //     }

// //     #[test]
// //     fn best_partitioning_constraint0() {
// //         let cs = [
// //             LengthEqualTo(10),
// //             LengthEqualTo(0),
// //             LengthEqualTo(1),
// //             LengthEqualTo(10),
// //             LengthEqualTo(2),
// //         ];
// //         assert_eq!(best_partitioning_constraint(&cs), Some(LengthEqualTo(10)))
// //     }

// //     #[test]
// //     fn best_partitioning_constraint1() {
// //         use Constraint::*;
// //         let cs = [
// //             LengthEqualTo(0),
// //             LengthEqualTo(1),
// //             LengthEqualTo(2),
// //             LengthGreaterThanOrEqualTo(2),
// //             LengthEqualTo(3),
// //             LengthEqualTo(4),
// //             LengthEqualTo(5),
// //             LengthGreaterThanOrEqualTo(5),
// //             LengthEqualTo(6),
// //             LengthEqualTo(7),
// //             LengthEqualTo(8),
// //             LengthGreaterThanOrEqualTo(8),
// //             LengthEqualTo(9),
// //             LengthEqualTo(10),
// //         ];
// //         assert_eq!(best_partitioning_constraint(&cs), Some(LengthGreaterThanOrEqualTo(5)))
// //     }

// //     // #[test]
// //     // fn count_encompassing_constraints0() {
// //     //     let c = Constraint::LengthEqualTo(10);
// //     //     let cs = [Constraint::LengthEqualTo(10)];
// //     //     assert_eq!(count_encompassing_constraints(&c, &cs), 1);
// //     // }

// //     // #[test]
// //     // fn count_encompassing_constraints1() {
// //     //     let c = Constraint::LengthEqualTo(10);
// //     //     let cs = [Constraint::LengthEqualTo(12)];
// //     //     assert_eq!(count_encompassing_constraints(&c, &cs), 0);
// //     // }

// //     // #[test]
// //     // fn count_encompassing_constraints2() {
// //     //     let c = Constraint::LengthEqualTo(10);
// //     //     let cs = [Constraint::LengthEqualTo(8)];
// //     //     assert_eq!(count_encompassing_constraints(&c, &cs), 0);
// //     // }

// //     // #[test]
// //     // fn count_encompassing_constraints3() {
// //     //     let c = Constraint::LengthEqualTo(10);
// //     //     let cs = [Constraint::LengthGreaterThanOrEqualTo(10)];
// //     //     assert_eq!(count_encompassing_constraints(&c, &cs), 1);
// //     // }

// //     // #[test]
// //     // fn count_encompassing_constraints4() {
// //     //     let c = Constraint::LengthEqualTo(10);
// //     //     let cs = [Constraint::LengthGreaterThanOrEqualTo(2)];
// //     //     assert_eq!(count_encompassing_constraints(&c, &cs), 1);
// //     // }

// //     // #[test]
// //     // fn count_encompassing_constraints5() {
// //     //     let c = Constraint::LengthEqualTo(10);
// //     //     let cs = [Constraint::LengthGreaterThanOrEqualTo(20)];
// //     //     assert_eq!(count_encompassing_constraints(&c, &cs), 0);
// //     // }

// //     // #[test]
// //     // fn count_encompassing_constraints6() {
// //     //     let c = Constraint::LengthEqualTo(10);
// //     //     let cs = [
// //     //         Constraint::LengthGreaterThanOrEqualTo(2),  // yes
// //     //         Constraint::LengthEqualTo(5),               // no
// //     //         Constraint::LengthEqualTo(10),              // yes
// //     //         Constraint::LengthGreaterThanOrEqualTo(10), // yes
// //     //         Constraint::LengthGreaterThanOrEqualTo(20), // no
// //     //     ];
// //     //     assert_eq!(count_encompassing_constraints(&c, &cs), 3);
// //     // }
// // }

// // // pub fn wider_constraint(c1: Constraint, c2: Constraint) -> Option<Constraint> {
// // //     (c1 == c2)
// // //         .then(|| c1.clone() /* we could have used c2 here as well */)
// // //         .or_else(|| match (&c1, &c2) {
// // //             (Constraint::LengthEqualTo(le), Constraint::LengthGreaterThanOrEqualTo(lge)) => {
// // //                 (lge <= le).then(|| c2)
// // //             }
// // //             (Constraint::LengthGreaterThanOrEqualTo(lge), Constraint::LengthEqualTo(le)) => {
// // //                 (lge <= le).then(|| c1)
// // //             }
// // //             _ => None,
// // //         })
// // // }

// // // #[derive(Clone)]
// // // pub struct NoDotDot {
// // //     indexed_from_start: Vec<Constraint>,
// // // }

// // // #[derive(Clone)]
// // // pub struct OneDotDot {
// // //     indexed_from_start: Vec<Constraint>,
// // //     dot_dot: Box<Constraint>,
// // //     indexed_from_end: Vec<Constraint>,
// // // }

// // // #[derive(Clone)]
// // // pub enum CompoundConstraint {
// // //     NoDotDot(NoDotDot),
// // //     OneDotDot(OneDotDot),
// // // }

// // // #[derive(Clone)]
// // // pub enum Constraint {
// // //     Compound(CompoundConstraint),
// // //     Symbol(String),
// // // }

// // // fn remove_dot_dot(c: OneDotDot) -> NoDotDot {
// // //     NoDotDot {
// // //         indexed_from_start: c
// // //             .indexed_from_start
// // //             .iter()
// // //             .chain(c.indexed_from_end.iter().rev())
// // //             .cloned()
// // //             .collect(),
// // //     }
// // // }

// // // fn shared_constraints(c1: Constraint, c2: Constraint) -> Vec<Constraint> {
// // //     let mut result = Vec::new();
// // //     match (&c1, &c2) {
// // //         (Constraint::Compound(c1c), Constraint::Compound(c2c)) => match (c1c, c2c) {
// // //             (CompoundConstraint::NoDotDot(_), CompoundConstraint::NoDotDot(_)) => todo!(),
// // //             (CompoundConstraint::NoDotDot(_), CompoundConstraint::OneDotDot(_)) => todo!(),
// // //             (CompoundConstraint::OneDotDot(_), CompoundConstraint::NoDotDot(_)) => todo!(),
// // //             (CompoundConstraint::OneDotDot(_), CompoundConstraint::OneDotDot(_)) => todo!(),
// // //         },
// // //         (Constraint::Symbol(s1), Constraint::Symbol(s2)) => {
// // //             if s1 == s2 {
// // //                 result.push(c1);
// // //             }
// // //         }
// // //         _ => {}
// // //     }
// // //     result
// // // }

// // // pub fn add_checks_single(rule: Rule, buffer: &mut Vec<EqualityCheck>) {
// // //     let pattern = match rule {
// // //         Rule::Computation(r) => r.pattern,
// // //         Rule::FixedPointRule(r) => r.pattern,
// // //     };
// // //     match pattern {
// // //         SinglePattern::Compound(c) => {
// // //             // buffer.push(EqualityCheck::Length(c.as_ref().len()))
// // //         },
// // //         SinglePattern::Variable(_) => {
// // //             // Variables match anything, so there are no checks to add
// // //         },
// // //         SinglePattern::Symbol(s) => {
// // //             buffer.push(EqualityCheck::Symbol(s))
// // //         },
// // //     }
// // // }

// // // pub fn add_checks_multi(pattern: MultiPattern)

// // // impl Env {
// // //     pub fn add_rule(&mut self, rule: Rule) {
// // //         match rule {
// // //             Rule::Computation(rule) => todo!(),
// // //             Rule::FixedPointRule(rule) => {
// // //                 self.arrows.push(Arrow::Fixpoint(FixpointArrow { checks: todo!() }))
// // //             },
// // //         }
// // //     }
// // // }
