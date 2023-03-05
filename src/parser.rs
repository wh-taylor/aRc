use crate::tokens::{LexError, Token};

pub struct Parser {
    pub chars: Vec<char>,
    pub index: usize,
    pub token: Result<Token, LexError>,
}

impl Parser {
    pub fn new(code: String) -> Parser {
        let mut parser = Parser {
            chars: code.chars().collect(),
            index: 0,
            token: Ok(Token::EOF),
        };
        parser.iter_token();
        parser
    }
}
