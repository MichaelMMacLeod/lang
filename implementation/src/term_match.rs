use petgraph::prelude::Bfs;

use crate::rule::CustomRule;
use crate::term::Term;
use crate::term_graph::TermGraph;

pub struct RuleMatch {
    bindings: Vec<Binding>,
}

impl RuleMatch {
    fn match_custom_rule(r: &CustomRule, tg: &TermGraph, term: Term) -> Option<Self> {
        let mut pattern_bfs = Bfs::new(&r.pattern.graph, r.pattern.redex);
        let mut tg_bfs = Bfs::new(&tg.graph, term);
        let mut bindings = vec![];
        let mut matched = false;
        while let (Some(p), Some(t)) = (pattern_bfs.next(&r.pattern.graph), tg_bfs.next(&tg.graph)) {
            let (sr, st) = (&r.pattern.graph[p], &tg.graph[t]);
            if sr.scope_set.binds(&st.scope_set) {
                matched = true;
            }
        }
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

pub struct BoundTerms {
    terms: Vec<Term>,
    dot_dot_dot_depth: u8,
}