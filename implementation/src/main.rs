use num_bigint;
use num_bigint::BigInt;
use petgraph;
use petgraph::Directed;
use petgraph::graph::NodeIndex;
use sexp;
use unicode_segmentation;

use std::collections::HashSet;
use std::io;

fn main() {}

enum SingularUnscopedTerm {
    Num(Box<BigInt>),
    Sym(Box<String>),
    Rule(Box<Rule>),
    Env(Vec<Box<Rule>>),
    Delimiter(Box<Delimiter>),
    DelimitedTerm(Box<DelimitedTerm>),
    CompoundTerm,
}

struct Rule {
    pattern: Box<ScopedTerm>,
    result: Box<ScopedTerm>,
}

struct Delimiter(usize);

struct DelimitedTerm {
    delimiter: Box<Delimiter>,
    term: Box<Term>,
    catch: Box<Rule>,
}

struct ScopedTerm {
    scope_set: HashSet<Scope>,
    singular_unscoped_term: SingularUnscopedTerm,
}

struct Scope(usize);

struct TermGraph(petgraph::stable_graph::StableGraph<ScopedTerm, SubtermIndex, Directed, NodeIndexType>);

struct SubtermIndex(usize);

struct Term(NodeIndex<NodeIndexType>);

type NodeIndexType = usize;