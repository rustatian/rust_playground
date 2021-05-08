use crate::parsemath::token::Token;
use crate::parsemath::tokenizer::Tokenizer;
use std::string::ParseError;

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::Invalid),
        };

        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }
}
