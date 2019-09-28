use nom::branch::alt;
use nom::combinator::peek;
use nom::sequence::{pair, preceded, tuple};

use super::{bind, expr, util};
use crate::ast::{ExprAssert, ExprLetIn, ExprWith};
use crate::lexer::Tokens;
use crate::parser::partial::{
    expect_terminated, many_till_partial, map_partial, map_partial_spanned, pair_partial, Partial,
};
use crate::parser::{tokens, IResult};

pub fn with(input: Tokens) -> IResult<Partial<ExprWith>> {
    let delims = alt((tokens::semi, tokens::eof));
    let scope = alt((expr, util::error_expr_if(delims, "semicolon")));
    let with = expect_terminated(preceded(tokens::keyword_with, expr), tokens::semi);
    let stmt = pair_partial(map_partial(with, Box::new), map_partial(scope, Box::new));
    map_partial_spanned(stmt, |span, (with, body)| ExprWith::new(with, body, span))(input)
}

pub fn assert(input: Tokens) -> IResult<Partial<ExprAssert>> {
    let delims = alt((tokens::semi, tokens::eof));
    let cond = alt((expr, util::error_expr_if(delims, "semicolon")));
    let assert = expect_terminated(preceded(tokens::keyword_assert, cond), tokens::semi);
    let stmt = pair_partial(map_partial(assert, Box::new), map_partial(expr, Box::new));
    map_partial_spanned(stmt, |span, (cond, body)| ExprAssert::new(cond, body, span))(input)
}

pub fn let_in(input: Tokens) -> IResult<Partial<ExprLetIn>> {
    let binds = many_till_partial(bind::bind, tokens::keyword_in);
    let let_binds = expect_terminated(preceded(tokens::keyword_let, binds), tokens::keyword_in);
    let stmt = pair_partial(let_binds, map_partial(expr, Box::new));
    map_partial_spanned(stmt, |span, (binds, body)| {
        ExprLetIn::new(binds, body, span)
    })(input)
}
