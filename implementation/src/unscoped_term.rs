use sexp::Sexp;

use crate::singular_unscoped_term::SingularUnscopedTerm;

#[derive(Debug)]
pub enum UnscopedTerm {
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
