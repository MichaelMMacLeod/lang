use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
};

use crate::{compound::Compound, delimiter::Delimiter, env::Env, rule::Rule, symbol::Symbol};

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

#[derive(Hash)]
pub enum Term {
    Symbol(Symbol),
    Compound(Compound),
    Env(Env),
    Delimiter(Delimiter),
}

pub struct Storage {
    data: HashMap<u64, Term>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: HashMap::default(),
        }
    }

    pub fn insert(&mut self, t: Term) -> u64 {
        let mut h = DefaultHasher::new();
        t.hash(&mut h);
        let hv = h.finish();
        self.data.insert(hv, t);
        hv
    }
}
