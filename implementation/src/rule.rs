use crate::term_graph::TermGraph;

#[derive(Debug)]
pub struct Rule {
    pattern: TermGraph,
    result: TermGraph,
}
