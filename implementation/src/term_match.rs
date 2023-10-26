use petgraph::prelude::Bfs;

use crate::rule::CustomRule;
use crate::term::Term;
use crate::term_graph::TermGraph;

pub struct RuleMatch {
    bindings: Vec<Binding>,
}

impl RuleMatch {
    fn match_custom_rule(r: &CustomRule, tg: &TermGraph, term: Term) -> Option<Self> {
        // $t singular
        // -->
        // $t=singular
        //
        // singular1 singular2
        // -->
        // when singular1==singular2, $t=singular1
        //
        // (Rule (List ...) (List ...))
        // -->
        // (Rule (List))
        let mut pattern_bfs = Bfs::new(&r.pattern.graph, r.pattern.redex);
        let mut tg_bfs = Bfs::new(&tg.graph, term);
        let mut bindings = vec![];
        let mut entered_graphs = false;
        let mut matched = false;
        while let (Some(p), Some(t)) = (pattern_bfs.next(&r.pattern.graph), tg_bfs.next(&tg.graph))
        {
            entered_graphs = true;
            let (sr, st) = (&r.pattern.graph[p], &tg.graph[t]);
            if sr.scope_set.binds(&st.scope_set) {
            }
        }
        assert!(entered_graphs);
        if matched {
            Some(Self { bindings })
        } else {
            None
        }
    }
}

pub struct Binding {
    identifier: String,
    bound_terms: BoundTerms,
}

impl Binding {
    pub fn new(identifier: String, bound_terms: BoundTerms) -> Self {
        Self {
            identifier,
            bound_terms,
        }
    }
}

pub enum BoundTerms {
    BindsToOneTerm(Term),
    BindsToMultipleTerms(Vec<Term>),

    NotBoundUnderDotDotDots(Vec<Term>),
    OneMoreLevelOfDotDotDots(Box<BoundTerms>),
}

// pub struct BoundTerms {
//     // One vec for each '...' binding level. For example,
//     //   (List (List $x ...) ...)
//     // when matched against
//     //   (List (List 1 2 3) (List 4) (List 5 6))
//     // could result in
//     //   ($x ... ...) --> (1 2 3 4 5 6)
//     //   (($x ...) ...) --> ((1 2 3) (4) (5 6))

//     // ((($x ...) ...) ...)
//     // (((1) (2 3)) ((4 5) (6 7 8)))
//     // ($x ... ... ...) --> (1 2 3 4 5 6 7 8)
//     // (($x ...) ... ...) --> ((1))
//     terms: Vec<Box<BoundTerms>>,
// }
