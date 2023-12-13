use crate::{
    compound::Compound,
    lexer::Lexed,
    storage::{self, Storage, StorageKey, Term},
};

pub fn parse<I: IntoIterator<Item = Lexed>>(storage: &mut Storage, lexed: I) -> StorageKey {
    let mut stack: Vec<Vec<StorageKey>> = Vec::new();
    let mut lexed = lexed.into_iter();
    while let Some(l) = lexed.next() {
        match l {
            Lexed::Left => {
                stack.push(vec![]);
            }
            Lexed::Right => {
                if let Some(data) = stack.pop() {
                    let key = storage.insert(Term::Compound(Compound::new(data)));
                    if let Some(partial_compound) = stack.last_mut() {
                        partial_compound.push(key);
                    } else {
                        return key;
                    }
                } else {
                    panic!("Unexpected ')'");
                }
            }
            Lexed::Symbol(s) => {
                let key = storage.insert(Term::Symbol(s));
                if let Some(partial_compound) = stack.last_mut() {
                    partial_compound.push(key);
                } else {
                    return key;
                }
            }
        }
    }
    panic!("Expected ')'");
}

#[cfg(test)]
mod test {
    use crate::lexer::lex;

    use super::*;

    #[test]
    fn parse1() {
        let mut s = Storage::new();
        let k = parse(
            &mut s,
            lex(b"(for a b (swap (pair a b)) -> (pair b a))").unwrap().1,
        );
        s.println(k);
    }
}
