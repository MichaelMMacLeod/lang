use num_bigint;
use petgraph;
use petgraph::stable_graph::StableGraph;
use sexp;
use unicode_segmentation;

use std::collections::HashSet;
use std::fmt::Formatter;
use std::io;

use crate::rule::Rule;
use crate::term_graph::TermGraph;

mod continuation;
mod unscoped_term;
mod singular_unscoped_term;
mod env;
mod rule;
mod scope_set;
mod scoped_term;
mod term_graph;
mod term;
mod term_match;

fn main() {
    let source = "(let (x 10) (let (x 100) x))";
    let mut term_graph = TermGraph::try_from(source).unwrap();
    term_graph.env = Env::builtin();

    let bl = Rule::builtin_let();
    let term_graph = bl.pattern();
    println!("{:?}", term_graph);
}
use crate::env::Env;