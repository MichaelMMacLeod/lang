use std::collections::HashSet;

use crate::{
    compound::Compound,
    rule::{apply_rule, ComputationRule, Rule},
    storage::{Storage, StorageKey, Term},
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Env {
    // Each key must point to a Rule
    rules: Vec<Rule>,
}

// (env <rule> ...) -> <env>
pub fn compile_env(storage: &Storage, env: StorageKey) -> Option<Env> {
    match storage.get(env).unwrap() {
        Term::Compound(c) => {
            if let Term::Symbol(s) = storage.get(*c.keys().get(0)?)? {
                if s.data() == "env" {
                    let rules: Vec<Rule> = c.keys()[1..]
                        .iter()
                        .filter_map(|k| match storage.get(*k).unwrap() {
                            Term::Rule(r) => Some(r.clone()),
                            _ => None,
                        })
                        .collect();
                    (rules.len() == c.keys().len()).then(|| Env { rules })
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn apply_matching_rule(
    env: &Env,
    storage: &mut Storage,
    fixed_points: &mut HashSet<StorageKey>,
    term: StorageKey,
) -> Option<StorageKey> {
    env.rules
        .iter()
        .filter_map(|rule| apply_rule(rule, storage, term))
        .next()
        .or_else(|| match storage.get(term).unwrap() {
            Term::Compound(c) => {
                let c = c.clone();
                let mut success = false;
                let mut new_keys: Vec<StorageKey> = Vec::new();
                for k in c.keys() {
                    if !success && !fixed_points.contains(k) {
                        if let Some(result) = apply_matching_rule(env, storage, fixed_points, *k) {
                            if result == *k {
                                fixed_points.insert(result);
                                new_keys.push(*k);
                            } else {
                                success = true;
                                new_keys.push(result);
                            }
                        } else {
                            new_keys.push(*k);
                        }
                    } else {
                        new_keys.push(*k);
                    }
                }
                if success {
                    Some(storage.insert(Term::Compound(Compound::new(new_keys))))
                } else {
                    None
                }
            }
            _ => None,
        })
}

pub fn reduce_to_fixed_point(
    env: &Env,
    storage: &mut Storage,
    mut term: StorageKey,
) -> Option<StorageKey> {
    let mut fixed_points: HashSet<StorageKey> = HashSet::new();
    storage.println(term);
    while let Some(term2) = apply_matching_rule(env, storage, &mut fixed_points, term) {
        if term2 == term {
            return Some(term);
        }
        term = term2;
        storage.println(term);
    }
    return None;
}

#[cfg(test)]
mod test {
    use crate::{parser::read, rule::compile_rule};

    use super::*;

    #[test]
    fn apply_matching_rule1() {
        let mut s = Storage::new();

        let rule1 = {
            let r = read(&mut s, "(for x -> 10)").unwrap();
            compile_rule(&mut s, r)
        };
        let rule2 = {
            let r = read(&mut s, "(for (+ 10 10) -> 20)").unwrap();
            compile_rule(&mut s, r)
        };
        let rule3 = {
            let r = read(&mut s, "(for (+ 20 10) -> 30)").unwrap();
            compile_rule(&mut s, r)
        };
        let rule4 = {
            let r = read(&mut s, "(for 30 -> 30)").unwrap();
            compile_rule(&mut s, r)
        };

        let env = Env {
            rules: vec![rule1, rule2, rule3, rule4],
        };

        let term = read(&mut s, "(+ (+ x x) x)").unwrap();

        reduce_to_fixed_point(&env, &mut s, term).unwrap();
    }

    #[test]
    fn apply_matching_rule2() {
        let mut s = Storage::new();

        let rule1 = {
            let r = read(&mut s, "(for 0 -> 0)").unwrap();
            compile_rule(&mut s, r)
        };
        let rule2 = {
            let r = read(&mut s, "(for n (succ n) -> (succ n))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule3 = {
            let r = read(&mut s, "(for n (plus n 0) -> n)").unwrap();
            compile_rule(&mut s, r)
        };
        let rule4 = {
            let r = read(&mut s, "(for n m (plus n (succ m)) -> (plus (succ n) m))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule5 = {
            let r = read(&mut s, "(for 1 -> (succ 0))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule6 = {
            let r = read(&mut s, "(for 2 -> (succ 1))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule7 = {
            let r = read(&mut s, "(for 3 -> (succ 2))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule8 = {
            let r = read(&mut s, "(for n (multiply n 0) -> 0)").unwrap();
            compile_rule(&mut s, r)
        };
        let rule9 = {
            let r = read(
                &mut s,
                "(for n m (multiply n (succ m)) -> (plus n (multiply n m)))",
            )
            .unwrap();
            compile_rule(&mut s, r)
        };

        let env = Env {
            rules: vec![
                rule1, rule2, rule3, rule4, rule5, rule6, rule7, rule8, rule9
            ],
        };

        let term = read(&mut s, "(multiply 2 3)").unwrap();

        reduce_to_fixed_point(&env, &mut s, term).unwrap();
    }


    #[test]
    fn apply_matching_rule3() {
        let mut s = Storage::new();

        let rule1 = {
            let r = read(&mut s, "(for 0 -> 0)").unwrap();
            compile_rule(&mut s, r)
        };
        let rule2 = {
            let r = read(&mut s, "(for n (succ n) -> (succ n))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule3 = {
            let r = read(&mut s, "(for n (plus n .. 0) -> (plus n ..))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule4 = {
            let r = read(&mut s, "(for n0 n m (plus n0 n .. (succ m)) -> (plus (succ n0) n .. m))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule5 = {
            let r = read(&mut s, "(for 1 -> (succ 0))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule6 = {
            let r = read(&mut s, "(for 2 -> (succ 1))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule7 = {
            let r = read(&mut s, "(for 3 -> (succ 2))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule8 = {
            let r = read(&mut s, "(for n (multiply n 0) -> 0)").unwrap();
            compile_rule(&mut s, r)
        };
        let rule9 = {
            let r = read(
                &mut s,
                "(for n m (multiply n (succ m)) -> (plus n (multiply n m)))",
            )
            .unwrap();
            compile_rule(&mut s, r)
        };

        let env = Env {
            rules: vec![
                rule1, rule2, rule3, rule4, rule5, rule6, rule7, rule8, rule9
            ],
        };

        let term = read(&mut s, "(plus 3 3 3)").unwrap();

        reduce_to_fixed_point(&env, &mut s, term).unwrap();
    }
}
