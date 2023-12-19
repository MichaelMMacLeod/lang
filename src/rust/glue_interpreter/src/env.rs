use std::collections::{HashSet, VecDeque};

use crate::{
    compound::Compound,
    rule::{apply_rule, ComputationRule, Reduction, Rule},
    storage::{Storage, StorageKey, Term},
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Env {
    // Each key must point to a Rule
    rules: Vec<Rule>,
}

impl Env {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }
}

// // (env <rule> ...) -> <env>
// pub fn compile_env(storage: &Storage, env: StorageKey) -> Option<Env> {
//     match storage.get(env).unwrap() {
//         Term::Compound(c) => {
//             if let Term::Symbol(s) = storage.get(*c.keys().get(0)?)? {
//                 if s.data() == "env" {
//                     let rules: Vec<Rule> = c.keys()[1..]
//                         .iter()
//                         .filter_map(|k| match storage.get(*k).unwrap() {
//                             Term::Rule(r) => Some(r.clone()),
//                             _ => None,
//                         })
//                         .collect();
//                     (rules.len() == c.keys().len()).then(|| Env { rules })
//                 } else {
//                     None
//                 }
//             } else {
//                 None
//             }
//         }
//         _ => None,
//     }
// }

pub fn reduce_once(env: &Env, storage: &mut Storage, term: StorageKey) -> Option<Reduction> {
    if let result @ Some(Reduction::Fixpoint | Reduction::Reduced) =
        apply_matching_rule(env, storage, term)
    {
        result
    } else {
        apply_matching_rule_recursively(env, storage, term)
    }
}

pub fn apply_matching_rule(
    env: &Env,
    storage: &mut Storage,
    term: StorageKey,
) -> Option<Reduction> {
    env.rules
        .iter()
        .filter_map(|rule| apply_rule(rule, storage, term))
        .next()
}

pub fn apply_matching_rule_recursively(
    env: &Env,
    storage: &mut Storage,
    term: StorageKey,
) -> Option<Reduction> {
    let mut stack = Vec::from([term]);
    while let Some(term) = stack.pop() {
        match storage.get(term).unwrap() {
            Term::Compound(c) => stack.extend(c.keys()),
            _ => {}
        }
        if let Some(Reduction::Reduced) = apply_matching_rule(env, storage, term) {
            return Some(Reduction::Reduced);
        }
    }
    None
}

pub fn reduce(
    env: &Env,
    storage: &mut Storage,
    term: StorageKey,
    graph_syntax: bool,
) -> Option<Reduction> {
    let mut step = 0;
    loop {
        let reduction = reduce_once(env, storage, term);
        if let Some(Reduction::Fixpoint) | None = reduction {
            return reduction;
        }
        step += 1;
        print!("{step}.\t");
        storage.println(term, graph_syntax);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{parser::read, rule::compile_rule};

    fn test_reduction(rules: &[&str], input: &str, expected: &str) {
        // TODO: impl this by just evaling "(equal input expected)" and checking if it is fixpoint

        let mut storage = Storage::new();

        let environment = Env::new(
            rules
                .into_iter()
                .map(|rule| {
                    let rule = read(&mut storage, rule).unwrap();
                    compile_rule(&mut storage, rule)
                })
                .collect(),
        );

        let input = read(&mut storage, input).unwrap();
        let expected = read(&mut storage, expected).unwrap();

        let graph_syntax = false;

        print!("Reducing ");
        storage.println(input, graph_syntax);
        reduce(&environment, &mut storage, input, graph_syntax);

        // print!("Reducing ");
        // storage.println(expected, graph_syntax);
        // reduce_to_fixed_point(&environment, &mut storage, expected, graph_syntax);

        assert!(storage.is_fixed(&input));
        assert!(storage.terms_are_equal(input, expected));
    }

    #[test]
    fn immediate_fixpoint() {
        test_reduction(&["(for x -> x)"], "x", "x");
    }

    #[test]
    fn super_simple_hardcoded_arithmetic() {
        test_reduction(
            &[
                "(for x -> 10)",
                "(for (+ 10 10) -> 20)",
                "(for (+ 20 10) -> 30)",
                "(for 30 -> 30)",
            ],
            "(+ (+ x x) x)",
            "30",
        );
    }

    #[test]
    fn peano_two_times_two_times_two() {
        test_reduction(
            &[
                "(for 0 -> 0)",
                "(for n (S n) -> (S n))",
                "(for n (n + 0) -> n)",
                "(for n m (n + (S m)) -> ((S n) + m))",
                "(for 2 -> (S (S 0)))",
                "(for n (n * 2) -> (n + n))",
            ],
            "((2 * 2) * 2)",
            "(S (S (S (S (S (S (S (S 0))))))))",
        );
    }

    #[test]
    fn peano_fibonacci_six_is_eight() {
        test_reduction(
            &[
                // Peano natural number fixpoints
                "(for      0  ->    0)",
                "(for n (S n) -> (S n))",
                // Some arabic numerals for simplicity
                "(for 1 -> (S 0))",
                "(for 2 -> (S 1))",
                "(for 3 -> (S 2))",
                "(for 4 -> (S 3))",
                "(for 5 -> (S 4))",
                "(for 6 -> (S 5))",
                "(for 7 -> (S 6))",
                "(for 8 -> (S 7))",
                // Addition
                "(for n   (+ n    0)  ->       n)",
                "(for n m (+ n (S m)) -> (+ (S n) m))",
                // Fibonacci
                "(for   (fib       0)   ->    0)",
                "(for   (fib    (S 0))  -> (S 0))",
                "(for n (fib (S (S n))) -> (+ (fib    n) 
                                              (fib (S n))))",
                // Boolean fixpoints
                "(for true  -> true)",
                "(for false -> false)",
                // Equality
                "(for     (equal    0     0)  -> true)",
                "(for m   (equal (S m)    0)  -> false)",
                "(for n   (equal    0  (S n)) -> false)",
                "(for m n (equal (S m) (S n)) -> (equal m n))",
            ],
            "(equal (fib 6) 8)",
            "true",
        );
    }

    #[test]
    fn dot_dot_is_non_greedy() {
        test_reduction(
            &[
                "(for x (X x ..) -> (X x ..))",
                "(for x y (x .. | y ..) -> (X = x .. Y = y ..))",
            ],
            "(1 2 3 4 | a b c d e f g h i)",
            "(X = 1 2 3 4 Y = a b c d e f g h i)",
        );
    }

    #[test]
    fn swap_pairs() {
        test_reduction(
            &[
                "(for v (list v ..) -> (list v ..))",
                "(for a b (swap (list (a b) ..)) -> (list (b a) ..))",
            ],
            "(swap (list (1 a) (2 b) (3 c) (4 d) (5 e) (6 f) (7 g)))",
            "(list (a 1) (b 2) (c 3) (d 4) (e 5) (f 6) (g 7))",
        );
    }

    #[test]
    fn heads_and_tails() {
        test_reduction(
            &[
                "(for r (result: r ..) -> (result: r ..))",
                "(for v (list v ..) -> (list v ..))",
                "(for head tail 
                   (heads+tails (list (list head tail ..) ..)) -> 
                   (result:
                    heads = (list head ..) , 
                    tails = (list tail .. ..)))",
            ],
            "(heads+tails (list (list 1 2 3 4 5) (list a b c d e) (list ! @ # $ %)))",
            "(result: heads = (list 1 a !) , tails = (list 2 3 4 5 b c d e @ # $ %))",
        );
        test_reduction(
            &[
                "(for r (result: r ..) -> (result: r ..))",
                "(for v (list v ..) -> (list v ..))",
                "(for head tail 
                   (heads+tails (list (list head tail ..) ..)) -> 
                   (result:
                    heads = (list head ..) , 
                    tails = (list (list tail ..) ..)))",
            ],
            "(heads+tails (list (list 1 2 3 4 5) (list a b c d e) (list ! @ # $ %)))",
            "(result: heads = (list 1 a !) , tails = (list (list 2 3 4 5) (list b c d e) (list @ # $ %)))",
        );
    }

    #[test]
    fn radix_sort() {
        test_reduction(
            &[
                "(for zero one n nb nr other
                   (sort (zero .. <--0 (n .. nb | 0 nr ..) other .. 1--> one ..)) ->
                   (sort (zero .. (n .. | nb 0 nr ..) <--0 other .. 1--> one ..)))",
                "(for zero one n nb nr other
                   (sort (zero .. <--0 (n .. nb | 1 nr ..) other .. 1--> one ..)) ->
                   (sort (zero .. <--0 other .. 1--> (n .. | nb 1 nr ..) one ..)))",
                "(for zero one n nr other
                   (sort (zero .. <--0 (| 0 nr ..) other .. 1--> one ..)) ->
                   (sort (zero .. (0 nr ..) <--0 other .. 1--> one ..)))",
                "(for zero one n nr other
                   (sort (zero .. <--0 (| 1 nr ..) other .. 1--> one ..)) ->
                   (sort (zero .. <--0 other .. 1--> (1 nr ..) one ..)))",
                "(for zero0 zero one
                   (sort (zero0 zero .. <--0 1--> one ..)) ->
                   (append (sort (<--0 zero0 zero .. 1-->))
                           (sort (<--0 one .. 1-->))))",
                "(for zero one0 one
                   (sort (zero .. <--0 1--> one0 one ..)) ->
                   (append (sort (<--0 zero .. 1-->))
                           (sort (<--0 one0 one .. 1-->))))",
                "(for n (sort (<--0 (n ..) 1-->)) -> (list (n ..)))",
                "(for x y (append (list x ..) (list y ..)) -> (list x .. y ..))",
                "(for x (list x ..) -> (list x ..))",
                "(for n m (radix-sort (n .. m) ..) -> (sort (<--0 (n .. | m) .. 1-->)))",
            ],
            "(radix-sort (1 1 1) (1 1 0) (0 1 1) (0 1 0) (1 0 1) (1 0 0) (0 0 1) (0 0 0))",
            "(list (0 0 0) (1 0 0) (0 1 0) (1 1 0) (0 0 1) (1 0 1) (0 1 1) (1 1 1))",
        );
    }
}
