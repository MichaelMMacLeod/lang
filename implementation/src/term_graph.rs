use petgraph::Directed;
use sexp::Sexp;

use std::fmt::Formatter;

use crate::scoped_term::ScopedTerm;
use crate::singular_unscoped_term::SingularUnscopedTerm;
use crate::subterm_index::SubtermIndex;
use crate::term::Term;
use crate::unscoped_term::UnscopedTerm;

pub struct TermGraph {
    graph: TermGraphImpl,
    redex: Term,
}

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

type TermGraphImpl = petgraph::stable_graph::StableGraph<ScopedTerm, SubtermIndex, Directed, NodeIndexType>;

#[derive(Debug)]
pub enum TermGraphParseError {
    SexpParseError(Box<sexp::Error>),
}

impl From<Box<sexp::Error>> for TermGraphParseError {
    fn from(e: Box<sexp::Error>) -> Self {
        Self::SexpParseError(e)
    }
}

pub type NodeIndexType = usize;
