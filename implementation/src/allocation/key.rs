// Data types:
//
// Symbols
// BigInts
// Ratios
// Strings
// Continuation delimiters
// Continuations
// Continuation frames
// Rules
// Environments
// Compound terms
//
// In addition, each of these have a single scope on them
//
// For simplicity, we can convert to and from symbols and compound terms
// easily for the following types:
//
// BigInts
// Ratios
// Strings
// Continuations (easily convertible to rules)
// Continuation frames (already represented as compound terms?)
// Rules
// 
// This leaves us with
//
// Symbols
// Continuation delimiters (these must be unique, so we have to treat them specially)
// Environments (it would be easier to write the matching code if the rules were precompiled somehow)
// Compound terms
//
// For super simplicity, we can do the following:
//
// Store everything in HashMap<HashOfTermPlusScope, TermPlusScope>
//
// and reference things by their hash.

use crate::scope_set::ScopeSet;

pub struct Rule {
    variables: Vec<String>,
    
}

pub struct Environment;

pub struct Symbol {
    scopes: ScopeSet,
    data: String,
}

pub enum Term {
    Symbol(Symbol),
    Delimiter(usize),
    Environment(Environment),
    CompoundTerm()
}