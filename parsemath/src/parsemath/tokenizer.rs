use std::iter::Peekable;
use std::str::Chars;

pub struct Tokenizer<'a> {
    // peekable iterator over the string
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    // creates a new tokenizer using the arithmetic expression provided by the user
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }
}
