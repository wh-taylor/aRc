use crate::tokens::Token;

pub struct Parser {
    chars: Vec<char>,
    index: usize,
}

impl Parser {
    pub fn new(filename: String, code: String) -> Parser {
        Parser {
            chars: code.chars().collect(),
            index: 0
        }
    }

    fn lex_word(&mut self) -> Option<Token> {
        let mut word = String::new();
        loop {
            match self.chars[self.index..] {
                [x, ..] if x.is_alphabetic() || x.is_numeric() || x == '_' => {
                    word.push(x);
                    self.index += 1;
                },
                _ => break Some(Token::Identifier(word)),
            }
        }
    }

    fn lex_number(&mut self) -> Option<Token> {
        let mut word = String::new();
        loop {
            match self.chars[self.index..] {
                [x, ..] if !x.is_numeric() && x != '_' && x != '.' || x == '.' && word.contains('.') => break,
                [x, ..] if x == '.' => match self.chars[self.index + 1..] {
                    [n, ..] if n.is_numeric() || n == '_' || n == '.' => continue,
                    _ => break,
                }
                [x, ..] => {
                    word.push(x);
                    self.index += 1;
                }
                [..] => break,
            }
        }
        Some(Token::Number(word))
    }

    pub fn next_token(&mut self) -> Option<Token> {
        match self.chars[self.index..] {
            [] => None,
            ['-', '>', ..] => { self.index += 2; return Some(Token::Arrow); },
            ['=', '>', ..] => { self.index += 2; return Some(Token::BigArrow); },
            ['+', '-', ..] => { self.index += 2; return Some(Token::PlusMinus); },
            ['=', '=', ..] => { self.index += 2; return Some(Token::DoubleEqual); },
            ['<', '=', ..] => { self.index += 2; return Some(Token::LessThanEqual); },
            ['>', '=', ..] => { self.index += 2; return Some(Token::GreaterThanEqual); },
            ['!', '=', ..] => { self.index += 2; return Some(Token::BangEqual); },
            ['=', ..] => { self.index += 1; return Some(Token::Equal); },
            [':', ..] => { self.index += 1; return Some(Token::Colon); },
            ['(', ..] => { self.index += 1; return Some(Token::LeftParen); },
            [')', ..] => { self.index += 1; return Some(Token::RightParen); },
            ['{', ..] => { self.index += 1; return Some(Token::LeftBrace); },
            ['}', ..] => { self.index += 1; return Some(Token::RightBrace); },
            ['[', ..] => { self.index += 1; return Some(Token::LeftBracket); },
            [']', ..] => { self.index += 1; return Some(Token::RightBracket); },
            ['<', ..] => { self.index += 1; return Some(Token::LessThan); },
            ['>', ..] => { self.index += 1; return Some(Token::GreaterThan); },
            ['|', ..] => { self.index += 1; return Some(Token::Bar); },
            ['+', ..] => { self.index += 1; return Some(Token::Plus); },
            ['-', ..] => { self.index += 1; return Some(Token::Minus); },
            ['*', ..] => { self.index += 1; return Some(Token::Star); },
            ['/', ..] => { self.index += 1; return Some(Token::Slash); },
            ['^', ..] => { self.index += 1; return Some(Token::Caret); },
            ['%', ..] => { self.index += 1; return Some(Token::Percent); },
            ['!', ..] => { self.index += 1; return Some(Token::Bang); },
            ['.', ..] => { self.index += 1; return Some(Token::Dot); },
            [',', ..] => { self.index += 1; return Some(Token::Comma); },
            ['\'', ..] => { self.index += 1; return Some(Token::Apostrophe); },
            [x, ..] if x.is_alphabetic() || x == '_' => self.lex_word(),
            [x, ..] if x.is_numeric() => self.lex_number(),
            [..] => {
                self.index += 1;
                self.next_token()
            }
        }
    }
}
