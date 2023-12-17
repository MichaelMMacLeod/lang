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

pub fn apply_matching_rule_toplevel(
    env: &Env,
    storage: &mut Storage,
    term: StorageKey,
) -> Option<StorageKey> {
    if storage.is_fixed(&term) {
        Some(term)
    } else {
        let result = apply_matching_rule(env, storage, term);
        if storage.is_fixed(&term) {
            Some(term)
        } else {
            result
        }
    }
}

pub fn apply_matching_rule(
    env: &Env,
    storage: &mut Storage,
    term: StorageKey,
) -> Option<StorageKey> {
    env.rules
        .iter()
        .filter_map(|rule| {
            if let Some(k) = apply_rule(rule, storage, term) {
                if k == term {
                    None
                } else {
                    Some(k)
                }
            } else {
                None
            }
        })
        .next()
        .or_else(|| match storage.get(term).unwrap() {
            Term::Compound(c) => {
                let c = c.clone();
                let mut success = false;
                let mut new_keys: Vec<StorageKey> = Vec::new();
                for k in c.keys() {
                    if !success {
                        if let Some(result) = apply_matching_rule(env, storage, *k) {
                            success = true;
                            new_keys.push(result);
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
    storage.println(term);
    while let Some(term2) = apply_matching_rule_toplevel(env, storage, term) {
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
        // let rule2 = {
        //     let r = read(&mut s, "(for n (succ n) -> (succ n))").unwrap();
        //     compile_rule(&mut s, r)
        // };
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
        let rule7a = {
            let r = read(&mut s, "(for 3 -> (succ 2))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule7b = {
            let r = read(&mut s, "(for 4 -> (succ 3))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule7c = {
            let r = read(&mut s, "(for 5 -> (succ 4))").unwrap();
            compile_rule(&mut s, r)
        };
        let rule7d = {
            let r = read(&mut s, "(for 6 -> (succ 5))").unwrap();
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
                rule1, //  rule2,
                rule3, rule4, rule5, rule6, rule7a, rule7b, rule7c, rule7d, rule8, rule9,
            ],
        };

        let term = read(&mut s, "(plus 3 6)").unwrap();

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
        let rule3a = {
            let r = read(&mut s, "(for n (plus n) -> n)").unwrap();
            compile_rule(&mut s, r)
        };
        let rule4 = {
            let r = read(
                &mut s,
                "(for n0 n m (plus n0 n .. (succ m)) -> (plus (succ n0) n .. m))",
            )
            .unwrap();
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
                rule1, rule2, rule3, rule3a, rule4, rule5, rule6, rule7, rule8, rule9,
            ],
        };

        let term = read(&mut s, "(multiply 2 (plus 3 3 3))").unwrap();

        reduce_to_fixed_point(&env, &mut s, term).unwrap();
    }

    #[test]
    fn apply_matching_rule4() {
        let mut s = Storage::new();

        let mut rule = |rule_text| {
            let r = read(&mut s, rule_text).unwrap();
            compile_rule(&mut s, r)
        };

        let env = Env {
            rules: vec![
                rule("(for 0 -> 0)"),
                rule("(for n (succ n) -> (succ n))"),
                rule("(for 1 -> (succ 0))"),
                rule("(for 2 -> (succ 1))"),
                rule("(for 3 -> (succ 2))"),
                rule("(for 4 -> (succ 3))"),
                rule("(for 5 -> (succ 4))"),
                rule("(for 6 -> (succ 5))"),
                rule("(for n (+ n 0) -> n)"),
                rule("(for n m (+ n (succ m)) -> (+ (succ n) m))"),
                rule("(for (fibs 0) -> 0)"),
                rule("(for (fibs (succ 0)) -> (succ 0))"),
                rule("(for n (fibs (succ (succ n))) -> (+ (fibs n) (fibs (succ n))))"),
            ],
        };

        let term = read(&mut s, "(fibs 6)").unwrap();
        // let term = read(&mut s, "0").unwrap();

        reduce_to_fixed_point(&env, &mut s, term).unwrap();
    }

    #[test]
    fn apply_matching_rule5() {
        let mut s = Storage::new();

        let mut rule = |rule_text| {
            let r = read(&mut s, rule_text).unwrap();
            compile_rule(&mut s, r)
        };

        let env = Env {
            rules: vec![
                // Digits
                rule("(for 0 -> 0)"),
                rule("(for 1 -> 1)"),
                rule("(for 2 -> 2)"),
                rule("(for 3 -> 3)"),
                rule("(for 4 -> 4)"),
                rule("(for 5 -> 5)"),
                rule("(for 6 -> 6)"),
                rule("(for 7 -> 7)"),
                rule("(for 8 -> 8)"),
                rule("(for 9 -> 9)"),
                // Bits
                rule("(for n m (bits n m) -> (bits n m))"),
                // Converting 0-9 to bits
                rule("(for (to-bits 0) -> 0)"),
                rule("(for (to-bits 1) -> 1)"),
                rule("(for (to-bits 2) -> (bits 0 1))"),
                rule("(for (to-bits 3) -> (bits 1 1))"),
                rule("(for (to-bits 4) -> (bits 0 (bits 0 1)))"),
                rule("(for (to-bits 5) -> (bits 1 (bits 0 1)))"),
                rule("(for (to-bits 6) -> (bits 0 (bits 1 1)))"),
                rule("(for (to-bits 7) -> (bits 1 (bits 1 1)))"),
                rule("(for (to-bits 8) -> (bits 0 (bits 0 (bits 0 1))))"),
                rule("(for (to-bits 9) -> (bits 1 (bits 0 (bits 0 1))))"),
                // Converting 0-9 bits to 0-9
                rule("(for (from-bits 0) -> 0)"),
                rule("(for (from-bits 1) -> 1)"),
                rule("(for (from-bits (bits 0 1)) -> 2)"),
                rule("(for (from-bits (bits 1 1)) -> 3)"),
                rule("(for (from-bits (bits 0 (bits 0 1))) -> 4)"),
                rule("(for (from-bits (bits 1 (bits 0 1))) -> 5)"),
                rule("(for (from-bits (bits 0 (bits 1 1))) -> 6)"),
                rule("(for (from-bits (bits 1 (bits 1 1))) -> 7)"),
                rule("(for (from-bits (bits 0 (bits 0 (bits 0 1)))) -> 8)"),
                rule("(for (from-bits (bits 1 (bits 0 (bits 0 1)))) -> 9)"),
                // Storing result & carry using two bits
                rule("(for c r (result (bits r c)) -> r)"),
                rule("(for c r (carry (bits r c)) -> c)"),
                // Bit addition (with carry in and carry out)
                rule("(for (+ 0 0 0) -> (bits 0 0))"),
                rule("(for (+ 0 0 1) -> (bits 1 0))"),
                rule("(for (+ 0 1 0) -> (bits 0 1))"),
                rule("(for (+ 0 1 1) -> (bits 0 1))"),
                rule("(for (+ 1 0 0) -> (bits 1 0))"),
                rule("(for (+ 1 0 1) -> (bits 0 1))"),
                rule("(for (+ 1 1 0) -> (bits 0 1))"),
                rule("(for (+ 1 1 1) -> (bits 1 1))"),
                // Addition
                rule(
                    "(for n0 n m0 m c
                        (+ (bits n0 n)
                           (bits m0 m)
                           c) 
                        -> 
                        (bits (result (+ n0 m0 c)) 
                              (+ n m (carry (+ n0 m0 c)))))",
                ),
                // Generic operations on two binary numbers (with carry) base cases
                rule(
                    "(for n0 n c f
                        (f (bits n0 n) 0 c) 
                        -> 
                        (f (bits n0 n) (bits 0 0) c))",
                ),
                rule(
                    "(for n0 n c f
                        (f (bits n0 n) 1 c) 
                        -> 
                        (f (bits n0 n) (bits 1 0) c))",
                ),
                rule(
                    "(for n0 n c f
                        (f 0 (bits n0 n) c) 
                        -> 
                        (f (bits n0 n) (bits 0 0) c))",
                ),
                rule(
                    "(for n0 n c f
                        (f 1 (bits n0 n) c) 
                        -> 
                        (f (bits n0 n) (bits 1 0) c))",
                ),
                // Bit multiplication (with carry in and carry out)
                rule("(for (* 0 0 0) -> (bits 0 0))"),
                rule("(for (* 0 0 1) -> (bits 1 0))"),
                rule("(for (* 0 1 0) -> (bits 0 0))"),
                rule("(for (* 0 1 1) -> (bits 1 0))"),
                rule("(for (* 1 0 0) -> (bits 0 0))"),
                rule("(for (* 1 0 1) -> (bits 1 0))"),
                rule("(for (* 1 1 0) -> (bits 0 0))"),
                rule("(for (* 1 1 1) -> (bits 1 1))"),
            ],
        };

        let term = read(&mut s, "(from-bits (+ (to-bits 3) (to-bits 6) 0))").unwrap();

        reduce_to_fixed_point(&env, &mut s, term).unwrap();
    }

    #[test]
    fn apply_matching_rule6() {
        let mut s = Storage::new();

        let mut rule = |rule_text| {
            let r = read(&mut s, rule_text).unwrap();
            compile_rule(&mut s, r)
        };

        let env = Env {
            rules: vec![
                rule("(for x y ($x = (x ..) $y = (y ..)) -> ($x = (x ..) $y = (y ..)))"),
                rule("(for x y (x .. <--x|y--> y ..) -> ($x = (x ..) $y = (y ..)))"),
            ],
        };

        let term = read(&mut s, "(0 1 2 3 <--x|y--> 4 5 6 7 8 9)").unwrap();

        reduce_to_fixed_point(&env, &mut s, term).unwrap();
    }

    #[test]
    fn booleans() {
        let mut s = Storage::new();

        let mut rule = |rule_text| {
            let r = read(&mut s, rule_text).unwrap();
            compile_rule(&mut s, r)
        };

        let env = Env {
            rules: vec![
                rule("(for true  -> true)"),
                rule("(for false -> false)"),

                rule("(for (is-false false) -> true)"),
                rule("(for (is-false true)  -> false)"),

                rule("(for   (all-true)            -> true)"),
                rule("(for b (all-true false b ..) -> false)"),
                rule("(for b (all-true true  b ..) -> (all-true b ..))"),

                rule("(for   (all-false)            -> true)"),
                rule("(for b (all-false true  b ..) -> false)"),
                rule("(for b (all-false false b ..) -> (all-false b ..))"),

                rule("(for   (at-least-one-true)            -> false)"),
                rule("(for b (at-least-one-true true  b ..) -> true)"),
                rule("(for b (at-least-one-true false b ..) -> (at-least-one-true b ..))"),

                rule("(for   (at-least-one-false)            -> false)"),
                rule("(for b (at-least-one-false false b ..) -> true)"),
                rule("(for b (at-least-one-false true  b ..) -> (at-least-one-false b ..))"),

                rule("(for   (exactly-one-true)                  -> false)"),
                rule("(for   (exactly-one-true false)            -> false)"),
                rule("(for   (exactly-one-true true)             -> true)"),
                rule("(for b (exactly-one-true true  true  b ..) -> false)"),
                rule("(for b (exactly-one-true false false b ..) -> (exactly-one-true      b ..))"),
                rule("(for b (exactly-one-true true  false b ..) -> (exactly-one-true true b ..))"),
                rule("(for b (exactly-one-true false true  b ..) -> (exactly-one-true true b ..))"),

                rule("(for   (exactly-one-false)                  -> false)"),
                rule("(for   (exactly-one-false true)             -> false)"),
                rule("(for   (exactly-one-false false)            -> true)"),
                rule("(for b (exactly-one-false false false b ..) -> false)"),
                rule("(for b (exactly-one-false true  true  b ..) -> (exactly-one-false       b ..))"),
                rule("(for b (exactly-one-false false true  b ..) -> (exactly-one-false false b ..))"),
                rule("(for b (exactly-one-false true  false b ..) -> (exactly-one-false false b ..))"),

                rule("(for (if true  then true)  -> true)"),
                rule("(for (if false then true)  -> true)"),
                rule("(for (if false then false) -> true)"),
                rule("(for (if true  then false) -> false)"),
            ],
        };

        let term = read(&mut s, "(exactly-one-false false true (at-least-one-true false true))").unwrap();

        reduce_to_fixed_point(&env, &mut s, term).unwrap();
    }


    #[test]
    fn identification() {
        let mut s = Storage::new();

        let mut rule = |rule_text| {
            let r = read(&mut s, rule_text).unwrap();
            compile_rule(&mut s, r)
        };

        let env = Env {
            rules: vec![
                rule("(for 0 -> 0)"),
                rule("(for n (S n) -> (S n))"),
                rule("(for n (n + 0) -> n)"),
                rule("(for n m (n + (S m)) -> ((S n) + m))"),
                rule("(for 2 -> (S (S 0)))"),
                rule("(for n (n * 2) -> (n + n))"),
                rule("(for n (double n) -> (n + n))"),
            ],
        };

        let term = read(&mut s, "((double 2) * 2)").unwrap();

        reduce_to_fixed_point(&env, &mut s, term).unwrap();
    }
}
