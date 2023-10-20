use num_bigint::BigInt;
use sexp::Atom;

use crate::delimited_term::DelimitedTerm;
use crate::delimiter::Delimiter;

#[derive(Debug)]
pub enum SingularUnscopedTerm {
    Num(Box<BigInt>),
    Sym(Box<String>),
    Delimiter(Box<Delimiter>),
    DelimitedTerm(Box<DelimitedTerm>),
}

impl From<Atom> for SingularUnscopedTerm {
    fn from(atom: sexp::Atom) -> Self {
        use sexp::Atom;
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
