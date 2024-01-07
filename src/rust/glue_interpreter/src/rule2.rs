use crate::{
    storage::{Storage, StorageKey, Term},
    symbol::Symbol,
};

struct IndexedPredicate {
    indices: Vec<Index>,
    predicate: Predicate,
}

enum Predicate {
    // Predicates for symbols
    SymbolEqualTo(String),

    // Predicates for compound terms
    LengthEqualTo(usize),
    LengthGreaterThanOrEqualTo(usize),
}

enum Index {
    ZeroPlus(usize),
    LengthMinus(usize),
    Middle(MiddleIndices),
}

enum IndexedConstructorIndices {
    ZeroPlus(usize),
    LengthMinus(usize),
    Middle,
}

struct MiddleIndices {
    starting_at_zero_plus: usize,
    ending_at_length_minus: usize,
}

struct IndexedConstructor {
    indices: Vec<IndexedConstructorIndices>,
    constructor: Constructor,
}

enum Constructor {
    Symbol(String),
    Copy(Vec<Index>),
}

struct PredicateList {
    list: Vec<IndexedPredicate>,
}

struct ConstructorList {
    list: Vec<IndexedConstructor>,
}

fn t1() {
    // (for x (a (b x .. c) .. d) -> (e x .. .. f))
    let pattern_predicate = PredicateList {
        list: vec![
            IndexedPredicate {
                indices: vec![],
                predicate: Predicate::LengthGreaterThanOrEqualTo(2),
            },
            IndexedPredicate {
                indices: vec![Index::ZeroPlus(0)],
                predicate: Predicate::SymbolEqualTo("a".into()),
            },
            IndexedPredicate {
                indices: vec![Index::LengthMinus(1)],
                predicate: Predicate::SymbolEqualTo("d".into()),
            },
            IndexedPredicate {
                indices: vec![Index::Middle(MiddleIndices {
                    starting_at_zero_plus: 1,
                    ending_at_length_minus: 2,
                })],
                predicate: Predicate::LengthGreaterThanOrEqualTo(2),
            },
            IndexedPredicate {
                indices: vec![
                    Index::Middle(MiddleIndices {
                        starting_at_zero_plus: 1,
                        ending_at_length_minus: 2,
                    }),
                    Index::ZeroPlus(0),
                ],
                predicate: Predicate::SymbolEqualTo("b".into()),
            },
            IndexedPredicate {
                indices: vec![
                    Index::Middle(MiddleIndices {
                        starting_at_zero_plus: 1,
                        ending_at_length_minus: 2,
                    }),
                    Index::LengthMinus(1),
                ],
                predicate: Predicate::SymbolEqualTo("c".into()),
            },
        ],
    };
    let result_constructor = ConstructorList {
        list: vec![
            IndexedConstructor {
                indices: vec![IndexedConstructorIndices::ZeroPlus(0)],
                constructor: Constructor::Symbol("e".into()),
            },
            IndexedConstructor {
                indices: vec![IndexedConstructorIndices::LengthMinus(1)],
                constructor: Constructor::Symbol("f".into()),
            },
            IndexedConstructor {
                indices: vec![IndexedConstructorIndices::Middle],
                constructor: Constructor::Copy(vec![
                    Index::Middle(MiddleIndices {
                        starting_at_zero_plus: 1,
                        ending_at_length_minus: 2,
                    }),
                    Index::Middle(MiddleIndices {
                        starting_at_zero_plus: 1,
                        ending_at_length_minus: 2,
                    }),
                ]),
            },
        ],
    };
}

struct ActiveConstructionIndex {
    zero_plus: usize,
}

struct ActiveConstructionIndicesList {
    list: Vec<ActiveConstructionIndex>,
}

fn construct_with_list(storage: &mut Storage, constructors: &ConstructorList, k: StorageKey, list: ActiveConstructionIndicesList) {
    match constructors.list.first() {
        Some(ic) => match &ic.constructor {
            Constructor::Symbol(s) => {
                let s = storage.insert(Term::Symbol(Symbol::new(s.clone())));
            }
            Constructor::Copy(_) => todo!(),
        },
        None => {}
    }
}

// use std::{
//     cmp::{Ordering, Reverse},
//     collections::BinaryHeap,
// };

// use crate::{
//     rule::{MultiPattern, Rule, SinglePattern},
//     symbol::Symbol,
// };

// // pub enum EqualityCheck {
// //     Length(usize),
// //     Symbol(String),
// //     SubtermLocation(SubtermLocation, Box<EqualityCheck>),
// // }

// // pub struct SubtermLocation {
// //     indices: Vec<usize>,
// // }

// // pub enum Assignment {
// //     Length(usize),
// //     Symbol(Symbol),
// //     SubtermLocation(SubtermLocation, Box<Assignment>),
// // }

// // pub enum Arrow {
// //     Nonfixed(NonfixedArrow),
// //     Fixpoint(FixpointArrow),
// // }

// // pub struct NonfixedArrow {
// //     checks: Vec<EqualityCheck>,
// //     destination: Box<Node>,
// // }

// // pub struct FixpointArrow {
// //     checks: Vec<EqualityCheck>,
// // }

// // pub struct Node {
// //     assignments: Vec<Assignment>,
// //     arrows: Vec<Arrow>,
// // }

// // pub struct Env {
// //     arrows: Vec<Arrow>,
// // }

// #[derive(Clone)]
// pub struct ConstraintList {
//     list: Vec<IndexedConstraint>,
// }

// #[derive(Clone)]
// pub struct IndexedConstraint {
//     indices: Vec<Index>,
//     constraint: Constraint,
// }

// #[derive(Clone)]
// pub enum Index {
//     FromStart(usize),
//     LengthMinus(usize),
//     DotDot(DotDotIndex),
// }

// #[derive(Clone)]
// pub struct DotDotIndex {
//     greater_than: usize,
//     less_than_length_minus: usize,
// }

// #[derive(Clone, PartialEq, Eq, Debug)]
// pub enum Constraint {
//     SymbolEqualTo(String),
//     LengthEqualTo(usize),
//     LengthGreaterThanOrEqualTo(usize),
// }

// impl PartialOrd for Constraint {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         (self.eq(other))
//             .then(|| Ordering::Equal)
//             .or_else(|| match (&self, &other) {
//                 (Constraint::LengthEqualTo(le), Constraint::LengthGreaterThanOrEqualTo(lge)) => {
//                     Some(lge.cmp(le))
//                 }
//                 (Constraint::LengthGreaterThanOrEqualTo(lge), Constraint::LengthEqualTo(le)) => {
//                     Some(lge.cmp(le))
//                 }
//                 (
//                     Constraint::LengthGreaterThanOrEqualTo(lge1),
//                     Constraint::LengthGreaterThanOrEqualTo(lge2),
//                 ) => Some(lge2.cmp(lge1)),
//                 _ => None,
//             })
//     }
// }

// pub fn partition_count(c: &Constraint, constraints: &[Constraint]) -> usize {
//     constraints
//         .into_iter()
//         .filter(|c2| c2 <= &c)
//         .count()
//         .abs_diff(constraints.len())
// }

// // pub fn count_encompassing_constraints<'a, C: IntoIterator<Item = &'a Constraint>>(
// //     c: &Constraint,
// //     constraints: C,
// // ) -> usize {
// //     constraints.into_iter().filter(|c2| c <= c2).count()
// // }

// // pub fn partition_count<'a, C: IntoIterator<Item = &'a Constraint>>(
// //     c: &Constraint,
// //     constraints: C,
// // ) -> usize {
// //     constraints.into_iter().filter(|c2| c <= c2).count()
// // }

// pub fn best_partitioning_constraint(constraints: &[Constraint]) -> Option<Constraint> {
//     struct PartitionCount {
//         constraint: Constraint,
//         count: usize,
//     }
//     impl PartialEq for PartitionCount {
//         fn eq(&self, other: &Self) -> bool {
//             self.count == other.count
//         }
//     }
//     impl Eq for PartitionCount {}
//     impl PartialOrd for PartitionCount {
//         fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//             self.count.partial_cmp(&other.count)
//         }
//     }
//     impl Ord for PartitionCount {
//         fn cmp(&self, other: &Self) -> Ordering {
//             self.count.cmp(&other.count)
//         }
//     }
//     constraints
//         .into_iter()
//         .map(|c| {
//             Reverse(PartitionCount {
//                 constraint: c.clone(),
//                 count: partition_count(c, constraints),
//             })
//         })
//         .collect::<BinaryHeap<_>>()
//         .pop()
//         .map(|c| c.0.constraint)
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use Constraint::*;

//     #[test]
//     fn partition_count0() {
//         let cs = [
//             LengthEqualTo(10),
//             LengthEqualTo(0),
//             LengthEqualTo(1),
//             LengthEqualTo(10),
//             LengthEqualTo(2),
//         ];
//         assert!(LengthEqualTo(10) <= LengthEqualTo(10));
//         assert!(LengthEqualTo(10) <= LengthEqualTo(10));
//         // assert_eq!(partition_count(&LengthEqualTo(10), &cs), 1);
//     }

//     #[test]
//     fn best_partitioning_constraint0() {
//         let cs = [
//             LengthEqualTo(10),
//             LengthEqualTo(0),
//             LengthEqualTo(1),
//             LengthEqualTo(10),
//             LengthEqualTo(2),
//         ];
//         assert_eq!(best_partitioning_constraint(&cs), Some(LengthEqualTo(10)))
//     }

//     #[test]
//     fn best_partitioning_constraint1() {
//         use Constraint::*;
//         let cs = [
//             LengthEqualTo(0),
//             LengthEqualTo(1),
//             LengthEqualTo(2),
//             LengthGreaterThanOrEqualTo(2),
//             LengthEqualTo(3),
//             LengthEqualTo(4),
//             LengthEqualTo(5),
//             LengthGreaterThanOrEqualTo(5),
//             LengthEqualTo(6),
//             LengthEqualTo(7),
//             LengthEqualTo(8),
//             LengthGreaterThanOrEqualTo(8),
//             LengthEqualTo(9),
//             LengthEqualTo(10),
//         ];
//         assert_eq!(best_partitioning_constraint(&cs), Some(LengthGreaterThanOrEqualTo(5)))
//     }

//     // #[test]
//     // fn count_encompassing_constraints0() {
//     //     let c = Constraint::LengthEqualTo(10);
//     //     let cs = [Constraint::LengthEqualTo(10)];
//     //     assert_eq!(count_encompassing_constraints(&c, &cs), 1);
//     // }

//     // #[test]
//     // fn count_encompassing_constraints1() {
//     //     let c = Constraint::LengthEqualTo(10);
//     //     let cs = [Constraint::LengthEqualTo(12)];
//     //     assert_eq!(count_encompassing_constraints(&c, &cs), 0);
//     // }

//     // #[test]
//     // fn count_encompassing_constraints2() {
//     //     let c = Constraint::LengthEqualTo(10);
//     //     let cs = [Constraint::LengthEqualTo(8)];
//     //     assert_eq!(count_encompassing_constraints(&c, &cs), 0);
//     // }

//     // #[test]
//     // fn count_encompassing_constraints3() {
//     //     let c = Constraint::LengthEqualTo(10);
//     //     let cs = [Constraint::LengthGreaterThanOrEqualTo(10)];
//     //     assert_eq!(count_encompassing_constraints(&c, &cs), 1);
//     // }

//     // #[test]
//     // fn count_encompassing_constraints4() {
//     //     let c = Constraint::LengthEqualTo(10);
//     //     let cs = [Constraint::LengthGreaterThanOrEqualTo(2)];
//     //     assert_eq!(count_encompassing_constraints(&c, &cs), 1);
//     // }

//     // #[test]
//     // fn count_encompassing_constraints5() {
//     //     let c = Constraint::LengthEqualTo(10);
//     //     let cs = [Constraint::LengthGreaterThanOrEqualTo(20)];
//     //     assert_eq!(count_encompassing_constraints(&c, &cs), 0);
//     // }

//     // #[test]
//     // fn count_encompassing_constraints6() {
//     //     let c = Constraint::LengthEqualTo(10);
//     //     let cs = [
//     //         Constraint::LengthGreaterThanOrEqualTo(2),  // yes
//     //         Constraint::LengthEqualTo(5),               // no
//     //         Constraint::LengthEqualTo(10),              // yes
//     //         Constraint::LengthGreaterThanOrEqualTo(10), // yes
//     //         Constraint::LengthGreaterThanOrEqualTo(20), // no
//     //     ];
//     //     assert_eq!(count_encompassing_constraints(&c, &cs), 3);
//     // }
// }

// // pub fn wider_constraint(c1: Constraint, c2: Constraint) -> Option<Constraint> {
// //     (c1 == c2)
// //         .then(|| c1.clone() /* we could have used c2 here as well */)
// //         .or_else(|| match (&c1, &c2) {
// //             (Constraint::LengthEqualTo(le), Constraint::LengthGreaterThanOrEqualTo(lge)) => {
// //                 (lge <= le).then(|| c2)
// //             }
// //             (Constraint::LengthGreaterThanOrEqualTo(lge), Constraint::LengthEqualTo(le)) => {
// //                 (lge <= le).then(|| c1)
// //             }
// //             _ => None,
// //         })
// // }

// // #[derive(Clone)]
// // pub struct NoDotDot {
// //     indexed_from_start: Vec<Constraint>,
// // }

// // #[derive(Clone)]
// // pub struct OneDotDot {
// //     indexed_from_start: Vec<Constraint>,
// //     dot_dot: Box<Constraint>,
// //     indexed_from_end: Vec<Constraint>,
// // }

// // #[derive(Clone)]
// // pub enum CompoundConstraint {
// //     NoDotDot(NoDotDot),
// //     OneDotDot(OneDotDot),
// // }

// // #[derive(Clone)]
// // pub enum Constraint {
// //     Compound(CompoundConstraint),
// //     Symbol(String),
// // }

// // fn remove_dot_dot(c: OneDotDot) -> NoDotDot {
// //     NoDotDot {
// //         indexed_from_start: c
// //             .indexed_from_start
// //             .iter()
// //             .chain(c.indexed_from_end.iter().rev())
// //             .cloned()
// //             .collect(),
// //     }
// // }

// // fn shared_constraints(c1: Constraint, c2: Constraint) -> Vec<Constraint> {
// //     let mut result = Vec::new();
// //     match (&c1, &c2) {
// //         (Constraint::Compound(c1c), Constraint::Compound(c2c)) => match (c1c, c2c) {
// //             (CompoundConstraint::NoDotDot(_), CompoundConstraint::NoDotDot(_)) => todo!(),
// //             (CompoundConstraint::NoDotDot(_), CompoundConstraint::OneDotDot(_)) => todo!(),
// //             (CompoundConstraint::OneDotDot(_), CompoundConstraint::NoDotDot(_)) => todo!(),
// //             (CompoundConstraint::OneDotDot(_), CompoundConstraint::OneDotDot(_)) => todo!(),
// //         },
// //         (Constraint::Symbol(s1), Constraint::Symbol(s2)) => {
// //             if s1 == s2 {
// //                 result.push(c1);
// //             }
// //         }
// //         _ => {}
// //     }
// //     result
// // }

// // pub fn add_checks_single(rule: Rule, buffer: &mut Vec<EqualityCheck>) {
// //     let pattern = match rule {
// //         Rule::Computation(r) => r.pattern,
// //         Rule::FixedPointRule(r) => r.pattern,
// //     };
// //     match pattern {
// //         SinglePattern::Compound(c) => {
// //             // buffer.push(EqualityCheck::Length(c.as_ref().len()))
// //         },
// //         SinglePattern::Variable(_) => {
// //             // Variables match anything, so there are no checks to add
// //         },
// //         SinglePattern::Symbol(s) => {
// //             buffer.push(EqualityCheck::Symbol(s))
// //         },
// //     }
// // }

// // pub fn add_checks_multi(pattern: MultiPattern)

// // impl Env {
// //     pub fn add_rule(&mut self, rule: Rule) {
// //         match rule {
// //             Rule::Computation(rule) => todo!(),
// //             Rule::FixedPointRule(rule) => {
// //                 self.arrows.push(Arrow::Fixpoint(FixpointArrow { checks: todo!() }))
// //             },
// //         }
// //     }
// // }
