use std::collections::{HashMap, HashSet};

use crate::storage::{Storage, StorageKey, Term};

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Rule {}

#[derive(Hash, PartialEq, Eq, Debug)]
enum SinglePattern {
    Compound(Box<MultiPattern>),
    Variable(String),
    Symbol(String),
}

#[derive(Hash, PartialEq, Eq, Debug)]
enum MultiPattern {
    Nothing,
    One(Box<One>),
    Many(Box<Many>),
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct One {
    sp: SinglePattern,
    mp: MultiPattern,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Many {
    sp: SinglePattern,
    mp: MultiPattern,
}

// TODO:
// - verify that every variable is not used more than once
// - verify that in "T ..", T must contain at least one variable

fn parse_rule_pattern_single(
    storage: &Storage,
    pattern: StorageKey,
    variables: &HashSet<String>,
) -> SinglePattern {
    match storage.get(pattern).unwrap() {
        Term::Symbol(s) => {
            if variables.contains(s.data()) {
                SinglePattern::Variable(s.data().clone())
            } else {
                SinglePattern::Symbol(s.data().clone())
            }
        }
        Term::Compound(c) => SinglePattern::Compound(Box::new(parse_rule_pattern_multi(
            storage,
            c.keys(),
            variables,
        ))),
        _ => panic!("invalid rule pattern"),
    }
}

fn parse_rule_pattern_multi(
    storage: &Storage,
    keys: &[StorageKey],
    variables: &HashSet<String>,
) -> MultiPattern {
    match keys.len() {
        0 => MultiPattern::Nothing,
        1 => MultiPattern::One(Box::new(One {
            sp: parse_rule_pattern_single(storage, keys[0], variables),
            mp: MultiPattern::Nothing,
        })),
        _ => {
            let k = keys[1];
            let is_dot_dotted = if let Term::Symbol(s) = storage.get(k).unwrap() {
                s.data() == ".."
            } else {
                false
            };
            if is_dot_dotted {
                MultiPattern::Many(Box::new(Many {
                    sp: parse_rule_pattern_single(storage, keys[0], variables),
                    mp: parse_rule_pattern_multi(storage, &keys[2..], variables),
                }))
            } else {
                MultiPattern::One(Box::new(One {
                    sp: parse_rule_pattern_single(storage, keys[0], variables),
                    mp: parse_rule_pattern_multi(storage, &keys[1..], variables),
                }))
            }
        }
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
enum Match {
    Leaf(StorageKey),
    Branches(Vec<Match>),
}

fn pattern_match_single(
    storage: &Storage,
    pattern: &SinglePattern,
    k: StorageKey,
) -> Option<HashMap<String, Match>> {
    match pattern {
        SinglePattern::Variable(v) => Some(HashMap::from([(v.clone(), Match::Leaf(k))])),
        SinglePattern::Symbol(s1) => {
            if let Term::Symbol(s2) = storage.get(k).unwrap() {
                if s1 == s2.data() {
                    Some(HashMap::new())
                } else {
                    None
                }
            } else {
                None
            }
        }
        SinglePattern::Compound(mp) => {
            if let Term::Compound(c) = storage.get(k).unwrap() {
                pattern_match_multi(storage, mp, c.keys())
            } else {
                None
            }
        }
    }
}

fn pattern_match_multi(
    storage: &Storage,
    pattern: &MultiPattern,
    ks: &[StorageKey],
) -> Option<HashMap<String, Match>> {
    match pattern {
        MultiPattern::Nothing => ks.is_empty().then(HashMap::new),
        MultiPattern::One(one) => {
            if let Some(k) = ks.get(0) {
                let One { sp, mp } = one.as_ref();
                pattern_match_single(storage, sp, *k)
                    .map(|matches1| {
                        pattern_match_multi(storage, mp, &ks[1..]).map(|matches2| {
                            let mut h = HashMap::new();
                            h.extend(matches1.into_iter());
                            h.extend(matches2.into_iter());
                            h
                        })
                    })
                    .flatten()
            } else {
                None
            }
        }
        MultiPattern::Many(many) => {
            let Many { sp, mp } = many.as_ref();
            let sp_matches = ks
                .iter()
                .map_while(|k| pattern_match_single(storage, sp, *k))
                .collect::<Vec<_>>();
            let combined_sp_matches = if !sp_matches.is_empty() {
                let mut branches: HashMap<String, Vec<Match>> = HashMap::new();
                for var in sp_matches[0].keys() {
                    for ht in sp_matches.iter() {
                        for (var2, m) in ht {
                            if var == var2 {
                                branches
                                    .entry(var.clone())
                                    .and_modify(|bs| bs.push(m.clone()))
                                    .or_insert(vec![m.clone()]);
                            } else {
                                break;
                            }
                        }
                    }
                }
                branches
                    .into_iter()
                    .map(|(k, v)| (k, Match::Branches(v)))
                    .collect()
            } else {
                HashMap::new()
            };
            pattern_match_multi(storage, mp, &ks[sp_matches.len()..]).map(|mp_matches| {
                let mut h = HashMap::new();
                h.extend(combined_sp_matches.into_iter());
                h.extend(mp_matches.into_iter());
                h
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{lexer::lex, parser::parse};

    use super::*;

    #[test]
    fn parse_rule_pattern_single1() {
        let mut s = Storage::new();
        let k = parse(&mut s, lex("(flatten (list (list xs ..) ..))").unwrap().1);
        let mut vs = HashSet::new();
        vs.insert("xs".into());
        let pattern = parse_rule_pattern_single(&s, k, &vs);
        dbg!(pattern);
    }

    #[test]
    fn pattern_match_single1() {
        let mut s = Storage::new();
        let k = parse(&mut s, lex("(flatten (list (list xs ..) ..))").unwrap().1);
        let mut vs = HashSet::new();
        vs.insert("xs".into());
        let pattern = parse_rule_pattern_single(&s, k, &vs);
        let value_k = parse(&mut s, lex("(flatten (list (list a b c) (list d e) (list)))").unwrap().1);
        let m = pattern_match_single(&s, &pattern, value_k);
        dbg!(m);
    }
}
