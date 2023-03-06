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
        self.parse_tuple()
    }

    fn parse_tuple(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_or()?;
        match self.token() {
            Ok(Token::Comma) => {},
            _ => return Ok(expr)
        }

        let mut elements = Vec::<Expression>::new();
        elements.push(expr);
        loop {
            match self.token() {
                Ok(Token::Comma) => {
                    self.iter_token();
                    
                    match self.token() {
                        Ok(Token::RightParen | Token::RightBrace | Token::RightBracket | Token::EOF) => break,
                        Ok(_) => {},
                        Err(e) => return Err(ParseError::LexError(e)),
                    }

                    let or = self.parse_or()?;
                    elements.push(or);
                },
                Ok(_) => break,
                Err(e) => return Err(ParseError::LexError(e)),
            }
        }
        Ok(Expression::Tuple(self.index, elements))
    }

    fn parse_or(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_and()?;
        loop {
            match self.token() {
                Ok(Token::Or) => {
                    self.iter_token();
                    let and = self.parse_and()?;
                    expr = Expression::Or(self.index, Box::new(expr), Box::new(and));
                },
                Ok(_) => break,
                Err(e) => return Err(ParseError::LexError(e)),
            }
        }
        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_comparison()?;
        loop {
            match self.token() {
                Ok(Token::And) => {
                    self.iter_token();
                    let comparison = self.parse_comparison()?;
                    expr = Expression::And(self.index, Box::new(expr), Box::new(comparison));
                },
                Ok(_) => break,
                Err(e) => return Err(ParseError::LexError(e)),
            }
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_addition()?;
        loop {
            match self.token() {
                Ok(Token::DoubleEqual) => {
                    self.iter_token();
                    let addition = self.parse_addition()?;
                    expr = Expression::Equal(self.index, Box::new(expr), Box::new(addition));
                },
                Ok(Token::BangEqual) => {
                    self.iter_token();
                    let addition = self.parse_addition()?;
                    expr = Expression::NotEqual(self.index, Box::new(expr), Box::new(addition));
                },
                Ok(Token::LessThan) => {
                    self.iter_token();
                    let addition = self.parse_addition()?;
                    expr = Expression::LessThan(self.index, Box::new(expr), Box::new(addition));
                },
                Ok(Token::GreaterThan) => {
                    self.iter_token();
                    let addition = self.parse_addition()?;
                    expr = Expression::GreaterThan(self.index, Box::new(expr), Box::new(addition));
                },
                Ok(Token::LessThanEqual) => {
                    self.iter_token();
                    let addition = self.parse_addition()?;
                    expr = Expression::LessThanEqual(self.index, Box::new(expr), Box::new(addition));
                },
                Ok(Token::GreaterThanEqual) => {
                    self.iter_token();
                    let addition = self.parse_addition()?;
                    expr = Expression::GreaterThanEqual(self.index, Box::new(expr), Box::new(addition));
                },
                Ok(_) => break,
                Err(e) => return Err(ParseError::LexError(e)),
            }
        }
        Ok(expr)
    }

    fn parse_addition(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_composition()?;
        loop {
            match self.token() {
                Ok(Token::Plus) => {
                    self.iter_token();
                    let composition = self.parse_composition()?;
                    expr = Expression::Add(self.index, Box::new(expr), Box::new(composition));
                }
                Ok(Token::Minus) => {
                    self.iter_token();
                    let composition = self.parse_composition()?;
                    expr = Expression::Subtract(self.index, Box::new(expr), Box::new(composition));
                }
                Ok(Token::PlusOrMinus) => {
                    self.iter_token();
                    let composition = self.parse_composition()?;
                    expr = Expression::Add(self.index, Box::new(expr), Box::new(Expression::PlusMinus(self.index, Box::new(composition))));
                }
                Ok(_) => break,
                Err(e) => return Err(ParseError::LexError(e)),
            }
        }
        Ok(expr)
    }

    fn parse_composition(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_multiplication()?;
        match self.token() {
            Ok(Token::Dot) => {
                self.iter_token();
                Ok(Expression::Compose(self.index, Box::new(expr), Box::new(self.parse_composition()?)))
            },
            Ok(_) => Ok(expr),
            Err(e) => Err(ParseError::LexError(e)),
        }
    }

    fn parse_multiplication(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_power()?;
        loop {
            match self.token() {
                Ok(Token::Star) => {
                    self.iter_token();
                    let power = self.parse_power()?;
                    expr = Expression::Multiply(self.index, Box::new(expr), Box::new(power));
                }
                Ok(Token::Slash) => {
                    self.iter_token();
                    let power = self.parse_power()?;
                    expr = Expression::Divide(self.index, Box::new(expr), Box::new(power));
                }
                Ok(_) => break,
                Err(e) => return Err(ParseError::LexError(e)),
            }
        }
        Ok(expr)
    }

    fn parse_power(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_prefix()?;
        match self.token() {
            Ok(Token::Caret) => {
                self.iter_token();
                Ok(Expression::Power(self.index, Box::new(expr), Box::new(self.parse_power()?)))
            },
            Ok(_) => Ok(expr),
            Err(e) => Err(ParseError::LexError(e)),
        }
    }

    fn parse_prefix(&mut self) -> Result<Expression, ParseError> {
        match self.token() {
            Ok(Token::Minus) => {
                self.iter_token();
                let expr = self.parse_prefix()?;
                Ok(Expression::Negate(self.index, Box::new(expr)))
            },
            Ok(Token::PlusOrMinus) => {
                self.iter_token();
                let expr = self.parse_prefix()?;
                Ok(Expression::PlusMinus(self.index, Box::new(expr)))
            },
            Ok(Token::Not) => {
                self.iter_token();
                let expr = self.parse_prefix()?;
                Ok(Expression::Not(self.index, Box::new(expr)))
            }
            Ok(_) => self.parse_implicit_multiplication(),
            Err(e) => Err(ParseError::LexError(e)),
        }
    }

    fn parse_implicit_multiplication(&mut self) -> Result<Expression, ParseError> {
        let expr = self.parse_postfix()?;
        match self.token() {
            Ok( Token::Number(_)
              | Token::Identifier(_)
              | Token::LeftParen) => Ok(Expression::Multiply(self.index, Box::new(expr), Box::new(self.parse_implicit_multiplication()?))),
            _ => Ok(expr),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression, ParseError> {
        let mut expr = self.parse_atom()?;
        loop {
            match self.token() {
                Ok(Token::Bang) => expr = Expression::Factorial(self.index, Box::new(expr)),
                Ok(Token::Percent) => expr = Expression::Percent(self.index, Box::new(expr)),
                Ok(_) => break,
                Err(e) => return Err(ParseError::LexError(e)),
            }
            self.iter_token();
        }
        Ok(expr)
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
        Ok(expr)
    }

    fn parse_parentheses(&mut self) -> Result<Expression, ParseError> {
        self.iter_token();
        let expr = self.parse()?;
        if let Ok(Token::RightParen) = self.token() {
            Ok(expr)
        } else {
            Err(ParseError::MissingClosingDelimiter)
        }
    }
}
