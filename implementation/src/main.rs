use num_bigint;
use num_bigint::BigInt;
use petgraph;
use petgraph::Directed;
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use sexp;
use sexp::{Atom, Sexp};
use unicode_segmentation;

use std::collections::HashSet;
use std::fmt::Formatter;
use std::io;

fn main() {
    let source = "(let (x 10) (let (x 100) x))";
    let term_graph = TermGraph::try_from(source).unwrap();
    println!("{:?}", term_graph);
}

#[derive(Debug)]
enum UnscopedTerm {
    Singular(SingularUnscopedTerm),
    Compound,
}

impl From<SingularUnscopedTerm> for UnscopedTerm {
    fn from(t: SingularUnscopedTerm) -> Self {
        Self::Singular(t)
    }
}

impl UnscopedTerm {
    fn singular(&self) -> Option<&SingularUnscopedTerm> {
        match self {
            Self::Singular(t) => Some(t),
            Self::Compound => None,
        }
    }
}

impl From<Sexp> for UnscopedTerm {
    fn from(sexp: Sexp) -> Self {
        match sexp {
            Sexp::Atom(atom) => Self::Singular(SingularUnscopedTerm::from(atom)),
            Sexp::List(_) => Self::Compound,
        }
    }
}

#[derive(Debug)]
enum SingularUnscopedTerm {
    Num(Box<BigInt>),
    Sym(Box<String>),
    Delimiter(Box<Delimiter>),
    DelimitedTerm(Box<DelimitedTerm>),
}

impl From<Atom> for SingularUnscopedTerm {
    fn from(atom: sexp::Atom) -> Self {
        use sexp::Atom;
        match atom {
            Atom::S(symbol) => {
                Self::Sym(Box::new(symbol))
            }
            Atom::I(integer) => {
                Self::Num(Box::new(num_bigint::BigInt::from(integer)))
            }
            Atom::F(_) => unimplemented!("float parsing")
        }
    }
}

#[derive(Debug)]
struct Env {
    rules: Vec<Box<Rule>>,
}

#[derive(Debug)]
struct Rule {
    scope_set: ScopeSet,
}

#[derive(Debug)]
struct Delimiter(usize);

#[derive(Debug)]
struct DelimitedTerm {
    delimiter: Box<Delimiter>,
    term: Box<Term>,
    catch: Box<Rule>,
}

#[derive(Debug)]
struct ScopeSet(HashSet<Scope>);

impl Default for ScopeSet {
    fn default() -> Self {
        Self(HashSet::default())
    }
}

struct ScopedTerm {
    scope_set: ScopeSet,
    unscoped_term: UnscopedTerm,
}

impl std::fmt::Debug for ScopedTerm {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self.unscoped_term)
    }
}

impl AsRef<UnscopedTerm> for ScopedTerm {
    fn as_ref(&self) -> &UnscopedTerm {
        &self.unscoped_term
    }
}

impl From<UnscopedTerm> for ScopedTerm {
    fn from(unscoped_term: UnscopedTerm) -> Self {
        Self {
            scope_set: ScopeSet::default(),
            unscoped_term,
        }
    }
}

impl From<SingularUnscopedTerm> for ScopedTerm {
    fn from(singular_unscoped_term: SingularUnscopedTerm) -> Self {
        Self {
            scope_set: ScopeSet::default(),
            unscoped_term: UnscopedTerm::from(singular_unscoped_term),
        }
    }
}

impl From<Sexp> for ScopedTerm {
    fn from(sexp: Sexp) -> Self {
        Self::from(UnscopedTerm::from(sexp))
    }
}

#[derive(Debug)]
struct Scope(usize);


struct TermGraph {
    graph: TermGraphImpl,
    redex: Term,
}

type TermGraphImpl = petgraph::stable_graph::StableGraph<ScopedTerm, SubtermIndex, Directed, NodeIndexType>;

impl std::fmt::Debug for TermGraph {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", petgraph::dot::Dot::new(&self.graph))
    }
}


impl TryFrom<&str> for TermGraph {
    type Error = TermGraphParseError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Ok(TermGraph::from(sexp::parse(str)?))
    }
}

impl From<Sexp> for TermGraph {
    fn from(sexp: Sexp) -> Self {
        let mut graph = TermGraphImpl::default();

        enum Step {
            InitialStep(Sexp),
            SubsequentStep(SubsequentStep),
        }

        struct SubsequentStep {
            sexp: Vec<Sexp>,
            parent: Parent,
        }

        impl SubsequentStep {
            fn push_on(self, stack: &mut Vec<Step>) {
                stack.push(Step::SubsequentStep(self));
            }
        }

        struct Parent {
            term: Term,
            current_subterm: usize,
        }

        enum ParsedSubterm {
            Singular(SingularUnscopedTerm),
            Compound(Vec<Sexp>),
        }

        impl From<Sexp> for ParsedSubterm {
            fn from(sexp: Sexp) -> Self {
                match sexp {
                    Sexp::Atom(a) => {
                        ParsedSubterm::Singular(SingularUnscopedTerm::from(a))
                    }
                    Sexp::List(l) => ParsedSubterm::Compound(l)
                }
            }
        }

        fn add_subterm(graph: &mut TermGraphImpl, sexp: Sexp) -> (Term, Option<SubsequentStep>) {
            let parsed_subterm = ParsedSubterm::from(sexp);
            match parsed_subterm {
                ParsedSubterm::Singular(u) => {
                    (graph.add_node(ScopedTerm::from(u)), None)
                }
                ParsedSubterm::Compound(l) => {
                    let redex = graph.add_node(ScopedTerm::from(UnscopedTerm::Compound));
                    let next_step = SubsequentStep {
                        sexp: l,
                        parent: Parent {
                            term: redex,
                            current_subterm: 0,
                        },
                    };
                    (redex, Some(next_step))
                }
            }
        }

        let mut stack = vec![Step::InitialStep(sexp)];
        let mut redex: Option<Term> = None;
        while let Some(step) = stack.pop() {
            match step {
                Step::InitialStep(sexp) => {
                    let (term, subsequent_step) = add_subterm(&mut graph, sexp);
                    redex = Some(term);
                    subsequent_step.map(|s| s.push_on(&mut stack));
                }
                Step::SubsequentStep(SubsequentStep { sexp, parent }) => {
                    if parent.current_subterm < sexp.len() {
                        stack.push(
                            Step::SubsequentStep(SubsequentStep {
                                sexp: sexp.clone(),
                                parent: Parent {
                                    term: parent.term,
                                    current_subterm: parent.current_subterm + 1,
                                },
                            })
                        );
                        let sexp = sexp[parent.current_subterm].clone();
                        let (term, subsequent_step) = add_subterm(&mut graph, sexp);
                        graph.add_edge(parent.term, term, SubtermIndex::from(parent.current_subterm));
                        subsequent_step.map(|s| s.push_on(&mut stack));
                    }
                }
            }
        }

        TermGraph {
            graph,
            redex: redex.unwrap(),
        }
    }
}

#[derive(Debug)]
enum TermGraphParseError {
    SexpParseError(Box<sexp::Error>),
}

impl From<Box<sexp::Error>> for TermGraphParseError {
    fn from(e: Box<sexp::Error>) -> Self {
        Self::SexpParseError(e)
    }
}

struct SubtermIndex(usize);

impl std::fmt::Debug for SubtermIndex {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self.0)
    }
}

impl From<usize> for SubtermIndex {
    fn from(u: usize) -> Self {
        Self(u)
    }
}

type Term = NodeIndex<NodeIndexType>;

type NodeIndexType = usize;