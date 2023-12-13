use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::Hasher,
};

use slotmap::{new_key_type, SlotMap};

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

    pub fn insert(&mut self, t: Term) -> StorageKey {
        self.data.insert(t)
    }

    pub fn println(&self, key: StorageKey) {
        self.print(key);
        println!();
    }

    pub fn print(&self, key: StorageKey) {
        match self.data.get(key).unwrap() {
            Term::Symbol(s) => print!("{}", String::from_utf8_lossy(s.data())),
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
            },
            Term::Rule(_) => print!("<rule>"),
            Term::Env(_) => print!("<env>"),
            Term::Delimiter(_) => print!("<delimiter>"),
        }
    }
}
