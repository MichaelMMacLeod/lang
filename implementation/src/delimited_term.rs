use crate::delimiter::Delimiter;
use crate::rule::Rule;
use crate::term::Term;

#[derive(Debug)]
pub struct DelimitedTerm {
    delimiter: Box<Delimiter>,
    term: Box<Term>,
    catch: Box<Rule>,
}
