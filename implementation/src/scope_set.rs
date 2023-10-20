use std::collections::HashSet;

use crate::scope::Scope;

#[derive(Debug)]
pub struct ScopeSet(HashSet<Scope>);

impl Default for ScopeSet {
    fn default() -> Self {
        Self(HashSet::default())
    }
}
