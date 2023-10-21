use crate::rule::Rule;

#[derive(Debug, Clone)]
pub struct Env {
    rules: Vec<Rule>,
}

impl Env {
    pub(crate) fn empty() -> Self {
        Self { rules: vec![] }
    }
    pub(crate) fn builtin() -> Self {
        Self { rules: vec![Rule::builtin_let()] }
    }
}
