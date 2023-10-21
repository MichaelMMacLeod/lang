use sexp::Sexp;

use crate::term_graph::TermGraph;

#[derive(Debug, Clone)]
pub enum Rule {
    Builtin(BuiltinRule),
    Custom(CustomRule),
}

impl Rule {
    pub fn pattern(&self) -> &TermGraph {
        match self {
            Self::Builtin(b) => &b.rule.pattern,
            Self::Custom(c) => &c.pattern,
        }
    }

    pub fn result(&self) -> &TermGraph {
        match self {
            Self::Builtin(b) => &b.rule.result,
            Self::Custom(c) => &c.result,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CustomRule {
    pub pattern: TermGraph,
    pub result: TermGraph,
}

#[derive(Debug, Clone)]
pub struct BuiltinRule {
    rule: CustomRule,
    kind: BuiltinRuleKind,
}

#[derive(Debug, Clone)]
pub enum BuiltinRuleKind {
    Let,
}

impl TryFrom<(&str, &str)> for CustomRule {
    type Error = Box<sexp::Error>;
    fn try_from((pattern, result): (&str, &str)) -> Result<Self, Self::Error> {
        Ok(Self::from((sexp::parse(pattern)?, sexp::parse(result)?)))
    }
}

impl From<(Sexp, Sexp)> for CustomRule {
    fn from((pattern, result): (Sexp, Sexp)) -> Self {
        Self {
            pattern: TermGraph::from(pattern),
            result: TermGraph::from(result),
        }
    }

    // fn match_term(&self, tg: TermGraph, term: Term) -> Option<Vec<Term>>
}

impl Rule {
    pub fn apply(&self, tg: &TermGraph) -> TermGraph {
        match self {
            Self::Builtin(b) => b.apply(tg),
            Self::Custom(c) => c.apply(tg),
        }
    }

    pub fn builtin_let() -> Self {
        let rule = CustomRule::try_from((
            "(let ($pattern $result) $body)",
            "$body"
        )).unwrap();
        Self::Builtin(BuiltinRule { rule, kind: BuiltinRuleKind::Let })
    }
}

impl BuiltinRule {
    pub fn apply(&self, tg: &TermGraph) -> TermGraph {
        match self.kind {
            BuiltinRuleKind::Let => self.apply_let(tg),
        }
    }

    pub fn apply_let(&self, tg: &TermGraph) -> TermGraph {
        // let (l, p, r) = self.match_let(tg);
        // assert(let.is_sym("let"))
        // pattern = tg.at([1, 0])
        // assert(pattern.is_any_sym())
        // result = tg.at([1, 1])
        // assert(result.is_any_sym())
        // new tg with (
        todo!()
    }
}

impl CustomRule {
    pub fn apply(&self, tg: &TermGraph) -> TermGraph {
        todo!()
    }
}
