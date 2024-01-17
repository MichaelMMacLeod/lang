use crate::rule::Rule;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Env {
    // Each key must point to a Rule
    rules: Vec<Rule>,
}

impl Env {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn rules(&self) -> &[Rule] {
        &self.rules
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

#[cfg(test)]
mod test {
    use std::time::Instant;

    use crate::{parser::read, rule::compile_rule, storage::Storage};

    fn test_reduction(rules: &[&str], input: &str, expected: &str) {
        let mut storage = Storage::new();

        let rules = rules
            .into_iter()
            .map(|rule| {
                let rule = read(&mut storage, rule).unwrap();
                compile_rule(&mut storage, rule)
            })
            .collect::<Vec<_>>();

        storage.add_rules(rules);

        let input = read(&mut storage, input).unwrap();
        let expected = read(&mut storage, expected).unwrap();

        let graph_syntax = false;

        print!("Reducing ");
        storage.println(input, graph_syntax);
        let now = Instant::now();
        storage.reduce(input);
        let elapsed = now.elapsed();
        println!("Elapsed time: {}ms ({}s)", elapsed.as_millis(), elapsed.as_secs());

        assert!(storage.is_fixed(&input));
        assert!(storage.terms_are_equal(input, expected));
    }

    // #[test]
    // fn builtin_rule_for() {
    //     test_reduction(
    //         &["(for x y (Pair x y) -> (Pair x y))"],
    //         "((for x y (mkpair x y) -> (Pair x y)) (mkpair Hello world!))",
    //         "(Pair Hello world!)",
    //     );
    // }

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

    #[test]
    fn speed0() {
        test_reduction(&[
          "(for x (F x ..) -> (F x ..))",
        "(for x y z ((x .. (y .. (z ..))) ..) -> (F x .. .. y .. .. z .. ..))"
      ],                   "((x0 (y0 (z0))) (x1 (y1 (z1))) (x2 (())) ((y2 ())) (((z2))) (((0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0))))"
, "(F x0 x1 x2 y0 y1 y2 z0 z1 z2 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0)");
    }

    #[test]
    fn brainfuck_quicksort() {
        // Implements a BrainFuck interpreter and runs a program
        // that sorts the list [3, 2, 1] into [1, 2, 3] via quicksort.
        //
        // The brainfuck program that does the sorting was copied from
        // https://www.codingame.com/playgrounds/50516/brainfuck-part-9---sort-arrays-with-bubble-and-quicksort/quicksort.
        let input = "
          (bf (input = ((S (S (S 0))) (S (S 0)) (S 0)))
            > , [ >
            + [ - > + < ] , ] > [ - > > > > > > > > > > > > >
            > > + < < < < < < < < < < < < < < < ] > > > > > >
            > > > > > > > > + [ [ - < + < + > > ] < [ - > + <
            ] < - [ - < < [ < ] < < < < < < < < < < < < [ - >
            > > > > > > > > > > > + < < < < < < < < < < < < ]
            > > > > > > > > > > > > [ > ] > ] > > > [ - < < +
            < + > > > ] < < [ - > > + < < ] < [ - < < [ < ] <
            < < < < < < < < + > > > > > > > > > > [ > ] > ] <
            < [ < ] < < < < < < < < < [ - < < [ < ] < [ - > +
            < ] > [ > ] > ] < < [ - > > > > + < < < < ] < [ [
            - > + > + < < ] > > [ - < < + > > ] < [ - > > [ >
            ] > > > > + < < < < < [ < ] < ] > > [ > ] > > [ -
            > + < < < + > > ] > [ - < + > ] > [ - < + > ] < <
            > > + < < [ - > [ - > ] > [ < < [ - ] > > - > > ]
            < < + < < ] > [ [ - ] < + > ] < [ - > + < ] < < [
            - > > + < < ] > > > [ - > - < < < < < [ < ] < < [
            - > > > [ > ] > > > > > > > > + < < < < < < < < <
            [ < ] < < ] > > > [ > ] > > > > > > > + [ - < + >
            ] < < < < < < < < [ < ] > [ [ - < + > ] > ] > [ -
            < + > ] > [ - < + > ] ] > [ - < < < + < < [ < ] <
            < [ - > > + < < ] > > [ > ] > > > > ] < < < < < [
            < ] < < ] > > > [ > ] > > > > > > > [ [ - < + < +
            > > ] < [ - > + < ] > [ - > > [ > ] > [ > ] > > +
            < < < [ < ] < < [ < ] < ] > > [ > ] > [ > ] > > >
            [ - < < < + > > > ] < < < [ - > > > + [ > ] > + <
            < [ < ] < ] > > [ - < + > ] < [ - > > [ > ] > > +
            < < < [ < ] < ] > > [ > ] > [ [ - < + > ] > ] < <
            [ < ] < < < [ < ] < [ < ] < ] < < < < < < [ [ - >
            > > > > > > > [ > ] > [ > ] > > > [ > ] > > + < <
            < [ < ] < < < [ < ] < [ < ] < < < < < < < ] > > >
            > [ - > > > > [ > ] > [ > ] > + < < [ < ] < [ < ]
            < < < ] > > > > [ > ] > [ > ] > > > [ - < + < + >
            > ] < [ - > + < ] < + [ - > > [ > ] > + < < [ < ]
            < ] > > [ > ] > [ [ - < + > ] > ] < < [ < ] < < <
            [ < ] < [ < ] < < < < < < < ] < < [ < ] > [ [ - <
            < < + > > > ] > ] > > [ - < < < < < + > > > > > ]
            > > > [ - ] > > > > [ [ - < < < < < < < < < < < +
            > > > > > > > > > > > ] > ] > [ [ - < < < < < < <
            < < < < < + > > > > > > > > > > > > ] > ] > > > [
            - ] > [ - ] > [ [ - < < + > > ] > ] < < < [ < ] >
            ] < < < < < < < < < < < < < < < < [ < ] > [ . > ]
          )
        ";
        test_reduction(
            &[
                "(for x in
                   (bf (input = in) x ..)
                   ->
                   (program
                     (input = in)
                     (output = ())
                     (mem = (# 0))
                     (cmd = (read (()) x ..))))",
                // Read '['
                "(for c x
                   (read (c ..) [ x ..)
                   ->
                   (read (() c ..) x ..))",
                // Read ']'
                "(for c0 c c1 x
                   (read (c0 (c ..) c1 ..) ] x ..)
                   ->
                   (read ((c .. c0) c1 ..) x ..))",
                // Read '.'
                "(for c0 c x
                   (read ((c0 ..) c ..) . x ..)
                   ->
                   (read ((c0 .. .) c ..) x ..))",
                // Read ','
                "(for c0 c x
                   (read ((c0 ..) c ..) , x ..)
                   ->
                   (read ((c0 .. ,) c ..) x ..))",
                // Read '+'
                "(for c0 c x
                   (read ((c0 ..) c ..) + x ..)
                   ->
                   (read ((c0 .. +) c ..) x ..))",
                // Read '-'
                "(for c0 c x
                   (read ((c0 ..) c ..) - x ..)
                   ->
                   (read ((c0 .. -) c ..) x ..))",
                // Read '>'
                "(for c0 c x
                   (read ((c0 ..) c ..) > x ..)
                   ->
                   (read ((c0 .. >) c ..) x ..))",
                // Read '<'
                "(for c0 c x
                   (read ((c0 ..) c ..) < x ..)
                   ->
                   (read ((c0 .. <) c ..) x ..))",
                // Finish reading
                "(for c0 c x
                   (read ((c ..)))
                   ->
                   (# c ..))",
                // Programs terminate when there are no commands on the stack
                "(for d in out
                   (program
                     (input = in)
                     (output = out)
                     (mem = d))
                   ->
                   (result = out))",
                "(for v (result = v) -> (result = v))",
                // When the instruction pointer (#) has reached the end of the
                // instructions on the current stack frame (cmd = ....), it pops
                // the current stack frame and starts executing the instructions
                // below.
                "(for i stack d in out stack0
                   (program
                     (input = in)
                     (output = out)
                     (mem = (d ..))
                     (cmd = (i .. #))
                     stack ..)
                   ->
                   (program
                     (input = in)
                     (output = out)
                     (mem = (d ..))
                     stack ..))",
                // '>' moves the data pointer to the right and the instruction
                // pointer to the right.
                "(for ib ia db d d0 da stack in out stack0
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. # d d0 da ..))
                     (cmd = (ib .. # > ia ..))
                     stack ..)
                   ->
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. d # d0 da ..))
                     (cmd = (ib .. > # ia ..))
                     stack ..))",
                // '>', when there are no more values in memory, allocates a fresh
                // zero on the right and moves to it.
                "(for ib ia db d stack in out stack0
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. # d))
                     (cmd = (ib .. # > ia ..))
                     stack ..)
                   ->
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. d # 0))
                     (cmd = (ib .. > # ia ..))
                     stack ..))",
                // '<' moves the data pointer to the left and the instruction
                // pointer to the *right*.
                "(for ib ia db d d0 da stack in out stack0
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. d0 # d da ..))
                     (cmd = (ib .. # < ia ..))
                     stack ..)
                   ->
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. # d0 d da ..))
                     (cmd = (ib .. < # ia ..))
                     stack ..))",
                // '+' increments the value at the data pointer (implemented here
                // using Peano arithmetic) and moves the instruction pointer to
                // the right.
                "(for ib ia db d da stack in out stack0
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. # d da ..))
                     (cmd = (ib .. # + ia ..))
                     stack ..)
                   ->
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. # (S d) da ..))
                     (cmd = (ib .. + # ia ..))
                     stack ..))",
                // '-' decrements the value at the data pointer (implemented here
                // using Peano arithmetic) and moves the instruction pointer to
                // the right.
                "(for ib ia db d da stack in out stack0
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. # (S d) da ..))
                     (cmd = (ib .. # - ia ..))
                     stack ..)
                   ->
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. # d da ..))
                     (cmd = (ib .. - # ia ..))
                     stack ..))",
                // '.' copies the value at the data pointer to the end of the output.
                "(for ib ia db d da stack in out stack0
                   (program
                     (input = in)
                     (output = (out ..))
                     (mem = (db .. # d da ..))
                     (cmd = (ib .. # . ia ..))
                     stack ..)
                   ->
                   (program
                     (input = in)
                     (output = (out .. d))
                     (mem = (db .. # d da ..))
                     (cmd = (ib .. . # ia ..))
                     stack ..))",
                // ',' removes a value from the start of the input and overwrites the
                // value at the data pointer with it.
                "(for ib ia db d da stack in0 in out stack0
                   (program
                     (input = (in0 in ..))
                     (output = out)
                     (mem = (db .. # d da ..))
                     (cmd = (ib .. # , ia ..))
                     stack ..)
                   ->
                   (program
                     (input = (in ..))
                     (output = out)
                     (mem = (db .. # in0 da ..))
                     (cmd = (ib .. , # ia ..))
                     stack ..))",
                // Second case for ','; if there's no input, use '0'.
                "(for ib ia db d da stack out
                   (program
                     (input = ())
                     (output = out)
                     (mem = (db .. # d da ..))
                     (cmd = (ib .. # , ia ..))
                     stack ..)
                   ->
                   (program
                     (input = ())
                     (output = out)
                     (mem = (db .. # 0 da ..))
                     (cmd = (ib .. , # ia ..))
                     stack ..))",
                // When the instruction pointer is at a list of instructions it
                // moves to the right if the value at the data pointer is zero.
                "(for ib ii ia db da stack in out
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. # 0 da ..))
                     (cmd = (ib .. # (ii ..) ia ..))
                     stack ..)
                   ->
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. # 0 da ..))
                     (cmd = (ib .. (ii ..) # ia ..))
                     stack ..))",
                // When the instruction pointer is at a list of instructions it
                // moves inside the list to the first instruction if the value at
                // the data pointer is nonzero.
                "(for ib ii ia db d da stack in out
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. # (S d) da ..))
                     (cmd = (ib .. # (ii ..) ia ..))
                     stack ..)
                   ->
                   (program
                     (input = in)
                     (output = out)
                     (mem = (db .. # (S d) da ..))
                     (cmd = (# ii ..))
                     (cmd = (ib .. # (ii ..) ia ..))
                     stack ..))",
            ],
            input,
            "(result = ((S 0) (S (S 0)) (S (S (S 0)))))",
        );
    }
}
