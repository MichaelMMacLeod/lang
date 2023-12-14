use std::collections::HashSet;

use crate::storage::{Storage, StorageKey, Term};

#[derive(Hash)]
pub struct Rule {}

enum SinglePattern {
    Compound(Box<MultiPattern>),
    Variable(StorageKey),
    Symbol(StorageKey),
}

enum MultiPattern {
    Nothing,
    One(Box<One>),
    Many(Box<Many>),
}

struct One {
    sp: SinglePattern,
    mp: MultiPattern,
}

struct Many {
    sp: SinglePattern,
    mp: MultiPattern,
}

fn parse_rule_pattern(storage: &mut Storage, pattern: StorageKey, variables: HashSet<String>) -> SinglePattern {
    match storage.get(pattern).unwrap() {
        Term::Symbol(_) => todo!(),
        Term::Compound(_) => todo!(),
        _ => panic!("invalid rule pattern"),
    }
}
