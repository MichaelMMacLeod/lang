use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct ScopeSet(HashSet<Scope>);

impl Default for ScopeSet {
    fn default() -> Self {
        Self(HashSet::default())
    }
}

impl ScopeSet {
    pub fn binds(&self, other: &Self) -> bool {
        self.0.is_subset(&other.0)
    }
}

type Scope = usize;
