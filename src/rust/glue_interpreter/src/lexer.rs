use nom::{
    branch::alt,
    bytes::complete::{is_not, take_while, take_while1},
    character::is_alphanumeric,
    multi::{many0, many1},
    sequence::{delimited, preceded},
    IResult,
};

use crate::symbol::Symbol;

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum Lexed {
    Left,
    Right,
    Symbol(Symbol),
}

fn lex_symbol(input: &[u8]) -> IResult<&[u8], Lexed> {
    let (i, o) = take_while1(|i| match i {
        b'(' => false,
        b')' => false,
        b' ' => false,
        _ => true,
    })(input)?;
    Ok((i, Lexed::Symbol(Symbol::new(o.to_vec()))))
}

fn lex_left(input: &[u8]) -> IResult<&[u8], Lexed> {
    use nom::character::complete::char;
    let (i, _) = char('(')(input)?;
    Ok((i, Lexed::Left))
}

fn lex_right(input: &[u8]) -> IResult<&[u8], Lexed> {
    use nom::character::complete::char;
    let (i, _) = char(')')(input)?;
    Ok((i, Lexed::Right))
}

pub fn lex(input: &[u8]) -> IResult<&[u8], Vec<Lexed>> {
    use nom::character::complete::char;
    many0(preceded(
        many0(char(' ')),
        alt((lex_symbol, lex_left, lex_right)),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex1() {
        let l = || Lexed::Left;
        let r = || Lexed::Right;
        let s = |b: &[u8]| Lexed::Symbol(Symbol::new(b.to_vec()));
        assert_eq!(
            lex(b"(for x (id x) -> x)"),
            Ok((
                b"".as_slice(),
                vec![
                    l(),
                    s(b"for"),
                    s(b"x"),
                    l(),
                    s(b"id"),
                    s(b"x"),
                    r(),
                    s(b"->"),
                    s(b"x"),
                    r()
                ]
            ))
        )
    }
}