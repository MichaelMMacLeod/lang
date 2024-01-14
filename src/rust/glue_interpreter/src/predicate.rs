use std::collections::HashSet;

use crate::{
    index::TermIndexN,
    storage::{Storage, StorageKey, Term},
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct PredicateSet {
    set: HashSet<IndexedPredicate>,
}

impl PredicateSet {
    fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }

    // fn matches(&self, storage: &Storage, k: StorageKey) -> bool {
    //     self.set.iter().all(|predicate| {
    //         let keys = get_indexed_all(storage, k, &predicate.indices);
    //         keys.into_iter().all(|key| {
    //             let term = storage.get(key).unwrap();
    //             match &predicate.predicate {
    //                 Predicate::SymbolEqualTo(string) => match term {
    //                     Term::Symbol(actual_symbol) => actual_symbol.data() == string,
    //                     _ => false,
    //                 },
    //                 Predicate::LengthEqualTo(size) => match term {
    //                     Term::Compound(actual_compound) => actual_compound.keys().len() == *size,
    //                     _ => false,
    //                 },
    //                 Predicate::LengthGreaterThanOrEqualTo(size) => match term {
    //                     Term::Compound(actual_compound) => actual_compound.keys().len() >= *size,
    //                     _ => false,
    //                 },
    //             }
    //         })
    //     })
    // }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct IndexedPredicate {
    index: Vec<TermIndexN>,
    predicate: Predicate,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Predicate {
    // Predicates for symbols
    SymbolEqualTo(String),

    // Predicates for compound terms
    LengthEqualTo(usize),
    LengthGreaterThanOrEqualTo(usize),
}
