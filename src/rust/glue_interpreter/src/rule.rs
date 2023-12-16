use std::collections::{HashMap, HashSet};

use crate::{
    compound::Compound,
    storage::{Storage, StorageKey, Term},
};

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
// - verify that in "T ..", T must contain at least one variable (?)

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
                        if let Some(m) = ht.get(var) {
                            branches
                                .entry(var.clone())
                                .and_modify(|bs| bs.push(m.clone()))
                                .or_insert(vec![m.clone()]);
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

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
enum ResultSinglePattern {
    Symbol(StorageKey),
    Variable(String),
    Compound(Vec<ResultPattern>),
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
enum ResultPattern {
    Single(Box<ResultSinglePattern>),
    Append(Box<ResultPattern>),
}

fn parse_rule_result_single(
    storage: &Storage,
    variables: &HashSet<String>,
    result: StorageKey,
) -> ResultSinglePattern {
    match storage.get(result).unwrap() {
        Term::Symbol(s) => {
            if variables.get(s.data()).is_some() {
                ResultSinglePattern::Variable(s.data().clone())
            } else {
                ResultSinglePattern::Symbol(result)
            }
        }
        Term::Compound(c) => {
            ResultSinglePattern::Compound(parse_rule_result_multi(storage, variables, c.keys()))
        }
        _ => panic!("invalid rule result"),
    }
}

fn is_dot_dots(storage: &Storage, k: StorageKey) -> bool {
    if let Term::Symbol(s) = storage.get(k).unwrap() {
        s.data() == ".."
    } else {
        false
    }
}

fn parse_rule_result_multi(
    storage: &Storage,
    variables: &HashSet<String>,
    keys: &[StorageKey],
) -> Vec<ResultPattern> {
    match keys.len() {
        0 => vec![],
        1 => vec![ResultPattern::Single(Box::new(parse_rule_result_single(
            storage, variables, keys[0],
        )))],
        _ => {
            let mut keys = keys;

            let mut parsed: Option<ResultPattern> = None;

            let is_variable = if let Term::Symbol(s) = storage.get(keys[0]).unwrap() {
                variables.contains(s.data())
            } else {
                false
            };

            while is_variable && keys.len() >= 2 && is_dot_dots(storage, keys[1]) {
                parsed = Some(ResultPattern::Append(Box::new(parsed.unwrap_or_else(
                    || {
                        ResultPattern::Single(Box::new(parse_rule_result_single(
                            storage, variables, keys[0],
                        )))
                    },
                ))));
                keys = &keys[1..];
            }

            let mut result: Vec<ResultPattern> = Vec::new();

            if let Some(p) = parsed {
                result.push(p);
            } else {
                result.push(ResultPattern::Single(Box::new(parse_rule_result_single(
                    storage, variables, keys[0],
                ))));
            }

            result.extend(parse_rule_result_multi(storage, variables, &keys[1..]));

            result
        }
    }
}

fn create_match_result_single(
    storage: &mut Storage,
    matches: &HashMap<String, Match>,
    pattern: &ResultSinglePattern,
) -> StorageKey {
    match pattern {
        ResultSinglePattern::Symbol(s) => *s,
        ResultSinglePattern::Variable(v) => {
            if let Match::Leaf(l) = matches[v] {
                l
            } else {
                panic!("bad result pattern");
            }
        }
        ResultSinglePattern::Compound(c) => {
            let keys: Vec<StorageKey> = create_match_result_multi(storage, matches, c);
            storage.insert(Term::Compound(Compound::new(keys)))
        }
    }
}

fn create_match_result_multi(
    storage: &mut Storage,
    matches: &HashMap<String, Match>,
    patterns: &[ResultPattern],
) -> Vec<StorageKey> {
    let mut result = Vec::new();

    for pattern in patterns {
        match pattern {
            ResultPattern::Single(s) => {
                result.push(create_match_result_single(storage, matches, s));
            }
            ResultPattern::Append(a) => {
                let matches = narrow_to_captured_variables(matches, a);
                let matches = split_branches(&matches);
                result.extend(matches.into_iter().flat_map(|matches| {
                    create_match_result_multi(storage, &matches, &[a.as_ref().clone()])
                }))
            }
        }
    }

    result
}

fn is_captured_variable(var: &String, pattern: &ResultPattern) -> bool {
    match pattern {
        ResultPattern::Single(s) => match s.as_ref() {
            ResultSinglePattern::Symbol(_) => false,
            ResultSinglePattern::Variable(v) => var == v,
            ResultSinglePattern::Compound(c) => c.iter().any(|p| is_captured_variable(var, p)),
        },
        ResultPattern::Append(a) => is_captured_variable(var, a),
    }
}

fn narrow_to_captured_variables(
    matches: &HashMap<String, Match>,
    pattern: &ResultPattern,
) -> HashMap<String, Match> {
    let mut result = HashMap::new();
    result.extend(
        matches
            .iter()
            .filter(|(k, v)| is_captured_variable(k, pattern))
            .map(|(k, v)| (k.clone(), v.clone())),
    );
    result
}

fn split_branches(matches: &HashMap<String, Match>) -> Vec<HashMap<String, Match>> {
    let mut result: Vec<HashMap<String, Match>> = Vec::new();

    let mut num_branches: Option<usize> = None;

    for (k, v) in matches {
        match v {
            Match::Leaf(_) => panic!("can't split leaf branches"),
            Match::Branches(b) => {
                if let None = num_branches {
                    num_branches = Some(b.len());
                }
                assert_eq!(num_branches.unwrap(), b.len());

                if result.len() == 0 {
                    for _ in 0..b.len() {
                        result.push(HashMap::new());
                    }
                }

                for (h, m) in result.iter_mut().zip(b) {
                    h.insert(k.clone(), m.clone());
                }
            }
        }
    }

    result
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
        let value_k = parse(
            &mut s,
            lex("(flatten (list (list a b c) (list d e) (list)))")
                .unwrap()
                .1,
        );
        let m = pattern_match_single(&s, &pattern, value_k);
        dbg!(m);
    }

    #[test]
    fn create_match_result_single1() {
        let mut s = Storage::new();

        let mut variables = HashSet::new();
        variables.insert("xs".into());

        let pattern_k = parse(&mut s, lex("(flatten (list (list xs ..) ..))").unwrap().1);
        let pattern = parse_rule_pattern_single(&s, pattern_k, &variables);

        let result_k = parse(&mut s, lex("(list xs .. ..)").unwrap().1);
        let result = parse_rule_result_single(&s, &variables, result_k);

        let value_k = parse(
            &mut s,
            lex("(flatten (list (list a b c) (list d e) (list)))")
                .unwrap()
                .1,
        );
        let m = pattern_match_single(&s, &pattern, value_k).unwrap();

        let r = create_match_result_single(&mut s, &m, &result);

        s.println(r);
    }
}
