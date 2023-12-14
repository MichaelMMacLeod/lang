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

fn lex_symbol(input: &str) -> IResult<&str, Lexed> {
    let (i, o) = take_while1(|i| match i {
        '(' => false,
        ')' => false,
        ' ' => false,
        '\n' => false,
        _ => true,
    })(input)?;
    Ok((i, Lexed::Symbol(Symbol::new(o.into()))))
}

fn lex_left(input: &str) -> IResult<&str, Lexed> {
    use nom::character::complete::char;
    let (i, _) = char('(')(input)?;
    Ok((i, Lexed::Left))
}

fn lex_right(input: &str) -> IResult<&str, Lexed> {
    use nom::character::complete::char;
    let (i, _) = char(')')(input)?;
    Ok((i, Lexed::Right))
}

pub fn lex(input: &str) -> IResult<&str, Vec<Lexed>> {
    use nom::character::complete::char;
    many0(preceded(
        many0(alt((char(' '), char('\n')))),
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
        let s = |b: &str| Lexed::Symbol(Symbol::new(b.into()));
        assert_eq!(
            lex("(for x (id x) -> x)"),
            Ok((
                "".into(),
                vec![
                    l(),
                    s("for"),
                    s("x"),
                    l(),
                    s("id"),
                    s("x"),
                    r(),
                    s("->"),
                    s("x"),
                    r()
                ]
            ))
        )
    }
}
