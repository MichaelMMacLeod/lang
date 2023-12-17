use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::Hasher,
};

use slotmap::{new_key_type, SlotMap};

use crate::{
    compound::Compound,
    delimiter::Delimiter,
    env::Env,
    rule::{ComputationRule, Rule, compile_rule},
    symbol::Symbol,
};

use std::hash::Hash;

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
    fixed_point_terms: HashSet<StorageKey>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: SlotMap::with_key(),
            fixed_point_terms: HashSet::new(),
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
            (Term::Compound(c1), Term::Compound(c2)) => {
                c1.keys().len() == c2.keys().len()
                    && c1
                        .keys()
                        .iter()
                        .zip(c2.keys().iter())
                        .all(|(t1, t2)| self.terms_are_equal(*t1, *t2))
            }
            (Term::Symbol(_), _) => false,
            (Term::Compound(_), _) => false,
            _ => todo!(),
        }
    }

    pub fn mark_as_fixed(&mut self, k: StorageKey) {
        self.fixed_point_terms.insert(k);
    }

    pub fn is_fixed(&self, k: &StorageKey) -> bool {
        self.fixed_point_terms.contains(k)
    }

    // pub fn apply_primitive_rule(&mut self, env: &Env, term: StorageKey) -> Option<StorageKey> {
    //     match self.get(term).unwrap() {
    //         Term::Compound(c) => c
    //             .keys()
    //             .first()
    //             .map(|k| {
    //                 if let Term::Symbol(s) = self.get(*k).unwrap() {
    //                     match s.data().as_str() {
    //                         "for" => compile_rule(&self, term),
    //                         "sequence" => todo!(),
    //                         "environment" => todo!(),
    //                         "environment-union" => todo!(),
    //                         "current-environment" => todo!(),
    //                         "frame" => todo!(),
    //                         "new-delimiter" => todo!(),
    //                         "delimit" => todo!(),
    //                         "abort" => todo!(),
    //                         "capture" => todo!(),
    //                         "let" => todo!(),
    //                         "letrec" => todo!(),
    //                         _ => None,
    //                     }
    //                 } else {
    //                     None
    //                 }
    //             })
    //             .flatten(),
    //         _ => None,
    //     }
    // }
}
