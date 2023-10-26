struct InternedSymbol {
    start: usize,
    length: usize,
}

struct SymbolInterner {
    symbols: HashSet<InternedSymbol>,
}