use petgraph::Directed;
use sexp::Sexp;

use std::fmt::Formatter;

use crate::env::Env;
use crate::scoped_term::ScopedTerm;
use crate::singular_unscoped_term::SingularUnscopedTerm;
use crate::term::Term;
use crate::unscoped_term::UnscopedTerm;

#[derive(Clone)]
pub struct TermGraph {
    pub graph: TermGraphImpl,
    pub redex: Term,
    pub env: Env,
}

impl std::fmt::Debug for TermGraph {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        #[allow(unused_imports)] // needed to stop intellij from removing it on reformat
        use petgraph::visit::NodeRef; // needed for n.id() below
        write!(fmt, "{:?}", petgraph::dot::Dot::with_attr_getters(
            &self.graph,
            &[],
            &|_, _| String::new(),
            &|_, n| if n.id() == self.redex {
                String::from("style=filled color=black fontcolor=white")
            } else {
                String::new()
            },
        ))
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
            current_subterm: u32,
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
                    if parent.current_subterm < u32::try_from(sexp.len()).unwrap() {
                        stack.push(
                            Step::SubsequentStep(SubsequentStep {
                                sexp: sexp.clone(),
                                parent: Parent {
                                    term: parent.term,
                                    current_subterm: parent.current_subterm + 1,
                                },
                            })
                        );
                        let sexp = sexp[usize::try_from(parent.current_subterm).unwrap()].clone();
                        let (term, subsequent_step) = add_subterm(&mut graph, sexp);
                        graph.add_edge(parent.term, term, parent.current_subterm);
                        subsequent_step.map(|s| s.push_on(&mut stack));
                    }
                }
            }
        }

        TermGraph {
            graph,
            redex: redex.unwrap(),
            env: Env::empty(),
        }
    }
}

type TermGraphImpl = petgraph::stable_graph::StableGraph<ScopedTerm, u32, Directed, u32>;

#[derive(Debug)]
pub enum TermGraphParseError {
    SexpParseError(Box<sexp::Error>),
}

impl From<Box<sexp::Error>> for TermGraphParseError {
    fn from(e: Box<sexp::Error>) -> Self {
        Self::SexpParseError(e)
    }
}

impl TermGraph {
    pub fn at<T: IntoIterator<Item=usize>>(&self, path: T) -> Option<ScopedTerm> {
        let mut term = self.redex;
        for i in path {
            term = self.graph.neighbors(term).nth(i)?;
        }
        Some(self.graph[term].clone())
    }

    pub fn do_one_reduction(&self) -> Self {
        todo!()
    }
}
