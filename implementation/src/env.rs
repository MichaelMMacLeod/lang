use crate::rule::Rule;

#[derive(Debug)]
struct Env {
    rules: Vec<Box<Rule>>,
}
