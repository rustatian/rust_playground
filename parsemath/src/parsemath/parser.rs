use crate::parsemath::token::{Token, OperPrec};
use crate::parsemath::tokenizer::Tokenizer;

#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidOperator("Invalid character".into())),
        };

        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }
   pub fn parse(&mut self) -> Result<Node, ParseError> {
       let ast = self.generate_ast(OperPrec::DefaultZero);
       match ast {
           Ok(ast) => Ok(ast),
           Err(e) => Err(e),
       }
   }
}
