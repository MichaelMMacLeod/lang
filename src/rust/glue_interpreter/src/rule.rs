use std::collections::HashSet;

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

#[cfg(test)]
mod test {
    use crate::{parser::parse, lexer::lex};

    use super::*;

    #[test]
    fn parse_rule_pattern_single1() {
        let mut s = Storage::new();
        let k = parse(
            &mut s,
            lex("(flatten (list (list xs ..) ..))").unwrap().1,
        );
        let mut vs = HashSet::new();
        vs.insert("xs".into());
        let pattern = parse_rule_pattern_single(&s, k, &vs);
        dbg!(pattern);
    }
}