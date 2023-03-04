use std::ops::Range;

use crate::concrete::*;
use crate::lex::Token;

use chumsky::prelude::*;

type Span = Range<usize>;
type Spanned<T> = (T, Span);

pub fn literal_parser() -> impl Parser<Token, Spanned<Literal>, extra::Err<Rich<char, Span>>> {
    select! {
    Token::Int(i) => Literal::Int(i),
    Token::Float(f) => Literal::Float(f),
    Token::String(s) => Literal::String(s),
    Token::True => Literal::Bool(true),
    Token::False => Literal::Bool(false),
    }
    .recover_with(skip_then_retry_until([]))
    .map_with_span(|tt, span| (tt, span))
}
