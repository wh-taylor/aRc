use crate::tokens::{Token, LexError};
use crate::parser::Parser;

impl Parser {
    fn lex_word(&mut self) -> Result<Token, LexError> {
        let mut word = String::new();
        loop {
            match self.chars[self.index..] {
                [x, ..] if x.is_alphabetic() || x.is_numeric() || x == '_' => {
                    word.push(x);
                    self.index += 1;
                },
                _ => break Ok(Token::Identifier(word)),
            }
        }
    }

    fn lex_number(&mut self) -> Result<Token, LexError> {
        let mut word = String::new();
        loop {
            match self.chars[self.index..] {
                [x, n, ..] if x == '.' && n.is_numeric() => word.push(x),
                [x, ..] if x == '.' => break,
                [x, ..] if !x.is_numeric() && x != '_' && x != '.' || x == '.' && word.contains('.') => break,
                [x, ..] => word.push(x),
                [..] => break,
            }
            self.index += 1;
        }
        Ok(Token::Number(word))
    }

    fn lex_symbol(&mut self) -> Result<Token, LexError> {
        match self.chars[self.index..] {
            [] => Ok(Token::EOF),
            ['+', '/', '-', ..] => { self.index += 2; return Ok(Token::PlusOrMinus); },
            ['-', '>', ..] => { self.index += 2; return Ok(Token::Arrow); },
            ['=', '>', ..] => { self.index += 2; return Ok(Token::BigArrow); },
            ['=', '=', ..] => { self.index += 2; return Ok(Token::DoubleEqual); },
            ['<', '=', ..] => { self.index += 2; return Ok(Token::LessThanEqual); },
            ['>', '=', ..] => { self.index += 2; return Ok(Token::GreaterThanEqual); },
            ['!', '=', ..] => { self.index += 2; return Ok(Token::BangEqual); },
            ['=', ..] => { self.index += 1; return Ok(Token::Equal); },
            [':', ..] => { self.index += 1; return Ok(Token::Colon); },
            ['(', ..] => { self.index += 1; return Ok(Token::LeftParen); },
            [')', ..] => { self.index += 1; return Ok(Token::RightParen); },
            ['{', ..] => { self.index += 1; return Ok(Token::LeftBrace); },
            ['}', ..] => { self.index += 1; return Ok(Token::RightBrace); },
            ['[', ..] => { self.index += 1; return Ok(Token::LeftBracket); },
            [']', ..] => { self.index += 1; return Ok(Token::RightBracket); },
            ['<', ..] => { self.index += 1; return Ok(Token::LessThan); },
            ['>', ..] => { self.index += 1; return Ok(Token::GreaterThan); },
            ['|', ..] => { self.index += 1; return Ok(Token::Bar); },
            ['+', ..] => { self.index += 1; return Ok(Token::Plus); },
            ['-', ..] => { self.index += 1; return Ok(Token::Minus); },
            ['*', ..] => { self.index += 1; return Ok(Token::Star); },
            ['/', ..] => { self.index += 1; return Ok(Token::Slash); },
            ['^', ..] => { self.index += 1; return Ok(Token::Caret); },
            ['%', ..] => { self.index += 1; return Ok(Token::Percent); },
            ['!', ..] => { self.index += 1; return Ok(Token::Bang); },
            ['.', ..] => { self.index += 1; return Ok(Token::Dot); },
            [',', ..] => { self.index += 1; return Ok(Token::Comma); },
            ['\'', ..] => { self.index += 1; return Ok(Token::Apostrophe); },
            [..] => { self.index += 1; Err(LexError::UnrecognizedSymbol) },
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        match self.chars[self.index..] {
            [] => Ok(Token::EOF),
            [x, ..] if x.is_alphabetic() || x == '_' => self.lex_word(),
            [x, ..] if x.is_numeric() => self.lex_number(),
            [x, ..] if x.is_whitespace() => {
                self.index += 1;
                self.next_token()
            }
            [..] => self.lex_symbol(),
        }
    }
}