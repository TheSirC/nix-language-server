pub use self::path::{path, path_template};

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::map;

use crate::ast::tokens::Literal;
use crate::parser::{map_spanned, IResult, Spanned};

mod path;

pub fn literal(input: Spanned) -> IResult<Literal> {
    let boolean = map_spanned(input, boolean, Literal::from);
    let null = map_spanned(input, null, Literal::from);
    let path = map_spanned(input, path, Literal::from);
    let path_template = map_spanned(input, path_template, Literal::from);
    alt((boolean, null, path, path_template))(input)
}

pub fn boolean(input: Spanned) -> IResult<bool> {
    let true_val = map(tag("true"), |_| true);
    let false_val = map(tag("false"), |_| false);
    alt((true_val, false_val))(input)
}

pub fn null(input: Spanned) -> IResult<()> {
    map(tag("null"), |_| ())(input)
}

fn take_n(n: usize) -> impl Fn(Spanned) -> IResult<Spanned> {
    move |input| take(n)(input)
}

#[cfg(test)]
mod tests {
    use nom::combinator::all_consuming;

    use super::*;

    #[test]
    fn boolean() {
        let string = Spanned::new("true");
        let (_, true_val) = all_consuming(boolean)(string).unwrap();
        assert_eq!(true_val, true);

        let string = Spanned::new("false");
        let (_, false_val) = all_consuming(boolean)(string).unwrap();
        assert_eq!(false_val, false);
    }

    #[test]
    fn null() {
        let string = Spanned::new("null");
        let (_, null) = all_consuming(null)(string).unwrap();
        assert_eq!(null, ());
    }
}