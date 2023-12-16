use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
};

use slotmap::{new_key_type, SlotMap};

use crate::{
    compound::Compound,
    delimiter::Delimiter,
    env::Env,
    rule::{ComputationRule, Rule},
    symbol::Symbol,
};

use std::hash::Hash;

// for
// seq
// env
// env-union
// this-env
// frame
// delimit
// delimiter
// abort
// let
// letrec

pub enum Term {
    Symbol(Symbol),
    Compound(Compound),
    Rule(Rule),
    Env(Env),
    Delimiter(Delimiter),
}

new_key_type! { pub struct StorageKey; }

pub struct Storage {
    data: SlotMap<StorageKey, Term>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: SlotMap::with_key(),
        }
    }

    pub fn get(&self, k: StorageKey) -> Option<&Term> {
        self.data.get(k)
    }

    pub fn insert(&mut self, t: Term) -> StorageKey {
        self.data.insert(t)
    }

    pub fn println(&self, key: StorageKey) {
        self.print(key);
        println!();
    }

    pub fn print(&self, key: StorageKey) {
        match self.data.get(key).unwrap() {
            Term::Symbol(s) => print!("{}", s.data()),
            Term::Compound(c) => {
                let keys = c.keys();
                if keys.is_empty() {
                    print!("()");
                } else {
                    print!("(");
                    for k in keys.iter().take(keys.len() - 1) {
                        self.print(*k);
                        print!(" ");
                    }
                    self.print(*keys.last().unwrap());
                    print!(")");
                }
            }
            Term::Rule(_) => print!("<rule>"),
            Term::Env(_) => print!("<env>"),
            Term::Delimiter(_) => print!("<delimiter>"),
        }
    }

    pub fn terms_are_equal(&self, t1: StorageKey, t2: StorageKey) -> bool {
        match (self.get(t1).unwrap(), self.get(t2).unwrap()) {
            (Term::Symbol(s1), Term::Symbol(s2)) => s1.data() == s2.data(),
            (Term::Compound(c1), Term::Compound(c2)) => c1
                .keys()
                .iter()
                .zip(c2.keys().iter())
                .all(|(t1, t2)| self.terms_are_equal(*t1, *t2)),
            (Term::Symbol(_), _) => false,
            (Term::Compound(_), _) => false,
            _ => todo!(),
        }
    }
}
