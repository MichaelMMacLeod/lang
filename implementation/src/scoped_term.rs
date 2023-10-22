use sexp::Sexp;

use std::fmt::Formatter;

use crate::scope_set::ScopeSet;
use crate::singular_unscoped_term::SingularUnscopedTerm;
use crate::unscoped_term::UnscopedTerm;

#[derive(Clone)]
pub struct ScopedTerm {
    pub scope_set: ScopeSet,
    pub unscoped_term: UnscopedTerm,
}

impl ScopedTerm {
    // pub fn binds(&self, other: &Self) -> Option<> {
    //     // $x <singular|compound> --> (Some (bind $x <singular|compound>))
    //     // <compound len=l1> <compound len=l2>

    //     // Doesn't bind if scopes don't match
    //     // case 1: $x and <T> --> bind $x to <T>
    //     // case 2: <X> and <T> for compound <X>, <T>: binds iff <X> and <T> have same # of subterms
    //     // case 3: <X> and <T> where 1 is singular and other is compound: doesn't match
    //     // case 4: <X> ... and 
    // }
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
