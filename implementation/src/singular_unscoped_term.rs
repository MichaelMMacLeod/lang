use num_bigint::BigInt;
use sexp::Atom;

use crate::continuation::Continuation;

#[derive(Debug, Clone)]
pub enum SingularUnscopedTerm {
    Num(Box<BigInt>),
    Sym(Box<String>),
    Continuation(Box<Continuation>),
}

impl From<Atom> for SingularUnscopedTerm {
    fn from(atom: sexp::Atom) -> Self {
        match atom {
            Atom::S(symbol) => {
                Self::Sym(Box::new(symbol))
            }
            Atom::I(integer) => {
                Self::Num(Box::new(num_bigint::BigInt::from(integer)))
            }
            Atom::F(_) => unimplemented!("float parsing")
        }
    }
}
