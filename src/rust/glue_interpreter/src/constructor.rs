use crate::index::TermIndex;

enum SingleConstructor {
    Copy(TermIndex),
    Symbol(String),
    Compound(Vec<CompoundElement>),
}

struct CompoundElement {
    single_constructor: SingleConstructor,
    dot_dot_count: usize,
}