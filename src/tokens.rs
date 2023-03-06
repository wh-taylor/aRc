#[derive(Clone, Debug)]
pub enum Token {
    Identifier(String),
    Number(String),
    EOF,
    Equal,
    Colon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Bar,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Percent,
    Bang,
    Dot,
    Arrow,
    BigArrow,
    PlusOrMinus,
    Comma,
    Apostrophe,
    DoubleEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    BangEqual,
}

#[derive(Clone, Debug)]
pub enum LexError {
    UnrecognizedSymbol,
}
