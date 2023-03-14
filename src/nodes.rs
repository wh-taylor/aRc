use crate::tokens::LexError;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Number(usize, isize, isize),
    ImaginaryConstant(usize),
    Boolean(bool),
    Variable(usize, String),
    Call(usize, Box<Expression>, Box<Expression>),
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
    Function(usize, Box<Expression>, Box<Expression>),
    Define(usize, Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum ParseError {
    LexError(LexError),
    NumberExpected,
    MissingClosingDelimiter,
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(_, a, b) if *b == 1 => write!(f, "{}", a),
            Self::Number(_, a, b) => write!(f, "{} / {}", a, b),
            Self::ImaginaryConstant(_) => write!(f, "i"),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Variable(_, v) => write!(f, "{}", v),
            Self::Call(_, x, y) => write!(f, "{} {}", x, y),
            Self::Percent(_, x) => write!(f, "{}%", x),
            Self::Factorial(_, x) => write!(f, "{}!", x),
            Self::Power(_, x, y) => write!(f, "{}^{}", x, y),
            Self::Compose(_, x, y) => write!(f, "{} . {}", x, y),
            Self::Multiply(_, x, y) => write!(f, "{} * {}", x, y),
            Self::Divide(_, x, y) => write!(f, "{} / {}", x, y),
            Self::Negate(_, x) => write!(f, "-{}", x),
            Self::PlusMinus(_, x) => write!(f, "+/-{}", x),
            Self::Add(_, x, y) => match *y.clone() {
                Self::PlusMinus(_, y) => write!(f, "{} +/- {}", x, y),
                _ => write!(f, "{} + {}", x, y),
            },
            Self::Subtract(_, x, y) => write!(f, "{} - {}", x, y),
            Self::Tuple(_, t) => write!(f, "({:?})", t),
            Self::Equal(_, x, y) => write!(f, "{} == {}", x, y),
            Self::NotEqual(_, x, y) => write!(f, "{} != {}", x, y),
            Self::LessThan(_, x, y) => write!(f, "{} < {}", x, y),
            Self::GreaterThan(_, x, y) => write!(f, "{} > {}", x, y),
            Self::LessThanEqual(_, x, y) => write!(f, "{} <= {}", x, y),
            Self::GreaterThanEqual(_, x, y) => write!(f, "{} >= {}", x, y),
            Self::And(_, x, y) => write!(f, "{} and {}", x, y),
            Self::Or(_, x, y) => write!(f, "{} or {}", x, y),
            Self::Not(_, x) => write!(f, "not {}", x),
            Self::Function(_, x, y) => write!(f, "{} => {}", x, y),
            Self::Define(_, x, y) => write!(f, "{} = {}", x, y),
        }
    }
}
