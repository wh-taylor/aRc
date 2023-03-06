use crate::tokens::LexError;

#[derive(Debug)]
pub enum Expression {
    Number(usize, String),
    Variable(usize, String),
    Function(usize, String, Vec<Expression>),
    Differentiate(usize, Box<Expression>),
    Percent(usize, Box<Expression>),
    Factorial(usize, Box<Expression>),
    Power(usize, Box<Expression>, Box<Expression>),
    Compose(usize, Box<Expression>, Box<Expression>),
    Multiply(usize, Box<Expression>, Box<Expression>),
    Divide(usize, Box<Expression>, Box<Expression>),
    Negate(usize, Box<Expression>),
    PlusMinus(usize, Box<Expression>),
    Add(usize, Box<Expression>, Box<Expression>),
    Subtract(usize, Box<Expression>, Box<Expression>),
    Tuple(usize, Vec<Expression>),
    Equal(usize, Box<Expression>, Box<Expression>),
    NotEqual(usize, Box<Expression>, Box<Expression>),
    LessThan(usize, Box<Expression>, Box<Expression>),
    GreaterThan(usize, Box<Expression>, Box<Expression>),
    LessThanEqual(usize, Box<Expression>, Box<Expression>),
    GreaterThanEqual(usize, Box<Expression>, Box<Expression>),
    And(usize, Box<Expression>, Box<Expression>),
    Or(usize, Box<Expression>, Box<Expression>),
    Not(usize, Box<Expression>),
}

#[derive(Debug)]
pub enum ParseError {
    LexError(LexError),
    NumberExpected,
    MissingClosingBracket,
}
