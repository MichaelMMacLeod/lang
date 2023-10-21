use num_bigint;
use petgraph;
use petgraph::stable_graph::StableGraph;
use sexp;
use unicode_segmentation;

use std::collections::HashSet;
use std::fmt::Formatter;
use std::io;
use term_graph::TermGraph;

mod continuation;
mod unscoped_term;
mod singular_unscoped_term;
mod env;
mod rule;
mod scope_set;
mod scoped_term;
mod scope;
mod term_graph;
mod subterm_index;
mod term;

fn main() {
    let source = "(let (x 10) (let (x 100) x))";
    let term_graph = TermGraph::try_from(source).unwrap();
    println!("{:?}", term_graph);
}
