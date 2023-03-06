use crate::tokens::{LexError, Token};
use crate::nodes::{ParseError, Expression};

#[derive(Clone)]
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

    pub fn parse(&mut self) -> Result<Expression, ParseError> {
        self.parse_atom()
    }

    fn parse_atom(&mut self) -> Result<Expression, ParseError> {
        let expr: Expression = match self.token() {
            Ok(Token::Number(n)) => Ok(Expression::Number(self.index, n)),
            Ok(Token::Identifier(id)) => Ok(Expression::Variable(self.index, id)),
            Ok(Token::LeftParen) => self.parse_parentheses(),
            Ok(_) => Err(ParseError::NumberExpected),
            Err(e) => Err(ParseError::LexError(e)),
        }?;
        self.iter_token();
        match self.token() {
            Ok(Token::Number(_) | Token::Identifier(_) | Token::LeftParen) => Ok(Expression::Multiply(self.index, Box::new(expr), Box::new(self.parse_atom()?))),
            _ => Ok(expr),
        }
    }

    fn parse_parentheses(&mut self) -> Result<Expression, ParseError> {
        self.iter_token();
        let expr = self.parse()?;
        if let Ok(Token::RightParen) = self.token() {
            Ok(expr)
        } else {
            Err(ParseError::MissingClosingParenthesis)
        }
    }
}
