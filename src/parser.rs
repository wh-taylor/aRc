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
            [x, ..] if x.is_alphabetic() || x == '_' => self.lex_word(),
            [x, ..] if x.is_numeric() => self.lex_number(),
            [..] => {
                self.index += 1;
                self.next_token()
            }
        }
    }
}
