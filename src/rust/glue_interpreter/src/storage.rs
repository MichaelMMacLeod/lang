use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::Hasher,
    num::NonZeroUsize,
};

use slotmap::{new_key_type, SlotMap};

use crate::{
    compound::Compound,
    delimiter::Delimiter,
    env::Env,
    rule::{compile_rule, ComputationRule, Rule},
    symbol::Symbol,
};

use std::hash::Hash;

#[derive(Clone)]
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

enum KeyUsageCount {
    Once,
    MoreThanOnce { graph_syntax_label: usize },
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

    pub fn get_mut(&mut self, k: StorageKey) -> Option<&mut Term> {
        self.data.get_mut(k)
    }

    // Replaces term at k with new_term, returning old term.
    pub fn replace(&mut self, k: StorageKey, new_term: Term) -> Term {
        let old_term = self.data.get_mut(k).unwrap();
        std::mem::replace(old_term, new_term)
    }

    pub fn insert(&mut self, t: Term) -> StorageKey {
        self.data.insert(t)
    }

    pub fn println(&self, key: StorageKey, graph_syntax: bool) {
        let labels = self.label_keys_used_more_than_once(key);
        let mut already_labeled = HashSet::new();
        self.print(key, &labels, &mut already_labeled, graph_syntax);
        println!();
    }

    pub fn print(
        &self,
        key: StorageKey,
        labels: &HashMap<StorageKey, usize>,
        already_labeled: &mut HashSet<StorageKey>,
        graph_syntax: bool,
    ) {
        if graph_syntax {
            if let Some(graph_syntax_label) = labels.get(&key) {
                if already_labeled.contains(&key) {
                    print!("#{graph_syntax_label}");
                    return;
                } else {
                    print!("#{graph_syntax_label}=");
                    already_labeled.insert(key);
                }
            }
        }
        match self.data.get(key).unwrap() {
            Term::Symbol(s) => print!("{}", s.data()),
            Term::Compound(c) => {
                let keys = c.keys();
                if keys.is_empty() {
                    print!("()");
                } else {
                    print!("(");
                    for k in keys.iter().take(keys.len() - 1) {
                        self.print(*k, labels, already_labeled, graph_syntax);
                        print!(" ");
                    }
                    self.print(*keys.last().unwrap(), labels, already_labeled, graph_syntax);
                    print!(")");
                }
            }
            Term::Rule(_) => print!("<rule>"),
            Term::Env(_) => print!("<env>"),
            Term::Delimiter(_) => print!("<delimiter>"),
        }
    }

    fn label_keys_used_more_than_once(&self, term: StorageKey) -> HashMap<StorageKey, usize> {
        let mut labels = HashMap::new();
        self.keys_used_more_than_once(term, &mut labels);
        labels
            .into_iter()
            .filter_map(|(k, v)| {
                if let KeyUsageCount::MoreThanOnce { graph_syntax_label } = v {
                    Some((k, graph_syntax_label))
                } else {
                    None
                }
            })
            .collect()
    }

    fn keys_used_more_than_once(
        &self,
        term: StorageKey,
        result: &mut HashMap<StorageKey, KeyUsageCount>,
    ) {
        let graph_syntax_label = result
            .iter()
            .filter_map(|(k, v)| {
                if let KeyUsageCount::MoreThanOnce { .. } = v {
                    Some(1)
                } else {
                    None
                }
            })
            .sum();
        result
            .entry(term)
            .and_modify(|u| {
                if let KeyUsageCount::Once = u {
                    *u = KeyUsageCount::MoreThanOnce { graph_syntax_label }
                }
            })
            .or_insert(KeyUsageCount::Once);
        if let Term::Compound(c) = self.get(term).unwrap() {
            for &term in c.keys() {
                self.keys_used_more_than_once(term, result);
            }
        };
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

    // Reduces the term at 'k' to a fixed point. After the call to
    // this function, 'k' points to the reduced term.
    // pub fn reduce(&mut self, k: StorageKey) {
    //     match self.get(k).unwrap() {
    //         Term::Symbol(_) => {},
    //         Term::Compound(_) => todo!(),
    //         Term::Rule(_) => todo!(),
    //         Term::Env(_) => todo!(),
    //         Term::Delimiter(_) => todo!(),
    //     }
    // }

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
