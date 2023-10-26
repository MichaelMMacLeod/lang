// use core::prelude::v1;

// use num_bigint::BigInt;
// use num_rational::Ratio;
// use proptest::{prelude::any, prop_assert, proptest, strategy::Strategy, test_runner::TestRunner, prop_assert_eq, prop_compose};

// #[derive(PartialEq, Eq, Debug)]
// pub enum Token {
//     Open,
//     Close,
//     Symbol(String),
// }

// #[derive(Debug)]
// pub enum ReadError {}

// pub fn read_tokens<'a, S: Into<&'a String>>(input: S) -> Result<Vec<Token>, ReadError> {
//     let mut v = vec![];
//     read_tokens_to_buffer(&mut v, input)?;
//     Ok(v)
// }

// pub fn read_tokens_to_buffer<'s, S: Into<&'s String>>(
//     buffer: &mut Vec<Token>,
//     input: S,
// ) -> Result<&mut Vec<Token>, ReadError> {
//     todo!()
// }

// static INTEGER_REGEX: &'static str = "-?[1-9][0-9]*";
// static RATIO_REGEX: &'static str = "-?[1-9][0-9]*/[1-9][0-9]*";

// // https://en.wikipedia.org/wiki/Unicode_character_property
// //
// // A symbol is parsed from one of the following:
// //
// //  1 - Not containing " and not containing white space or '(' or ')' or ';'
// //
// //  2 - Optionally one of (1) followed by " followed by ANY (possibly none)
// //      unicode code points followed by " followed by the first id (if it 
// //      exists)
// static SOME_BUT_NOT_ALL_SYMBOLS_REGEX: &'static str = "([^\"\\s();]+|#\"[^#]*\"#|multiline\"[^#]*\"multiline)";

// fn arb_term() -> impl Strategy<Value = Vec<Token>> {

// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_read_parens() {
//         let open = read_tokens(&"(".to_string()).expect("open failure");
//         let close = read_tokens(&")".to_string()).expect("close failure");
//         assert_eq!(open.len(), 1);
//         assert_eq!(close.len(), 1);
//         assert_eq!(&open[0], &Token::Open);
//         assert_eq!(&close[0], &Token::Close);
//     }
// }

// proptest! {
//     #[test]
//     fn test_read_symbol(input in SOME_BUT_NOT_ALL_SYMBOLS_REGEX) {
//         match read_tokens(&input) {
//             Ok(v) => {
//                 prop_assert_eq!(v.len(), 1);
//                 match &v[0] {
//                     Token::Symbol(s) => prop_assert_eq!(s, &input),
//                     _ => prop_assert!(false)
//                 }
//             },
//             _ => prop_assert!(false),
//         }
//     }
// }