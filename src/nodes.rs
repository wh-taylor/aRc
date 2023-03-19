use crate::tokens::LexError;
use Expression::*;

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
            Number(_, a, b) if *b == 1 => write!(f, "{}", a),
            Number(_, a, b) => write!(f, "{} / {}", a, b),
            ImaginaryConstant(_) => write!(f, "i"),
            Boolean(b) => write!(f, "{}", b),
            Variable(_, v) => write!(f, "{}", v),
            Call(_, x, y) => write!(f, "{} {}", x, y),
            Percent(_, x) => write!(f, "{}%", x),
            Factorial(_, x) => write!(f, "{}!", x),
            Power(_, x, y) => write!(f, "{}^{}", x, y),
            Compose(_, x, y) => write!(f, "{} . {}", x, y),
            Multiply(_, x, y) => write!(f, "{} * {}", x, y),
            Divide(_, x, y) => write!(f, "{} / {}", x, y),
            Negate(_, x) => write!(f, "-{}", x),
            PlusMinus(_, x) => write!(f, "+/-{}", x),
            Add(_, x, y) => match *y.clone() {
                PlusMinus(_, y) => write!(f, "{} +/- {}", x, y),
                _ => write!(f, "{} + {}", x, y),
            },
            Subtract(_, x, y) => write!(f, "{} - {}", x, y),
            Tuple(_, t) => write!(f, "({:?})", t),
            Equal(_, x, y) => write!(f, "{} == {}", x, y),
            NotEqual(_, x, y) => write!(f, "{} != {}", x, y),
            LessThan(_, x, y) => write!(f, "{} < {}", x, y),
            GreaterThan(_, x, y) => write!(f, "{} > {}", x, y),
            LessThanEqual(_, x, y) => write!(f, "{} <= {}", x, y),
            GreaterThanEqual(_, x, y) => write!(f, "{} >= {}", x, y),
            And(_, x, y) => write!(f, "{} and {}", x, y),
            Or(_, x, y) => write!(f, "{} or {}", x, y),
            Not(_, x) => write!(f, "not {}", x),
            Function(_, x, y) => write!(f, "{} => {}", x, y),
            Define(_, x, y) => write!(f, "{} = {}", x, y),
        }
    }
}

impl Expression {
    pub fn sub(&self, old: &Expression, new: &Expression) -> Box<Expression> {
        let expr = match (self.clone(), old.clone()) {
            (Variable(_, name), Variable(_, name2)) if name == name2 => new.clone(),
            (Number(_, _, _), _) => self.clone(),
            (ImaginaryConstant(_), _) => self.clone(),
            (Boolean(_), _) => self.clone(),
            (Variable(_, _), _) => self.clone(),
            (Call(i, a, b), _) => Call(i, a.sub(old, new), b.sub(old, new)),
            (Percent(i, a), _) => Percent(i, a.sub(old, new)),
            (Factorial(i, a), _) => Factorial(i, a.sub(old, new)),
            (Power(i, a, b), _) => Power(i, a.sub(old, new), b.sub(old, new)),
            (Compose(i, a, b), _) => Compose(i, a.sub(old, new), b.sub(old, new)),
            (Multiply(i, a, b), _) => Multiply(i, a.sub(old, new), b.sub(old, new)),
            (Divide(i, a, b), _) => Divide(i, a.sub(old, new), b.sub(old, new)),
            (Negate(i, a), _) => Negate(i, a.sub(old, new)),
            (PlusMinus(i, a), _) => PlusMinus(i, a.sub(old, new)),
            (Add(i, a, b), _) => Add(i, a.sub(old, new), b.sub(old, new)),
            (Subtract(i, a, b), _) => Subtract(i, a.sub(old, new), b.sub(old, new)),
            (Tuple(i, xs), _) => Tuple(i, xs.into_iter().map(|x| *x.sub(old, new)).collect()),
            (Equal(i, a, b), _) => Equal(i, a.sub(old, new), b.sub(old, new)),
            (NotEqual(i, a, b), _) => NotEqual(i, a.sub(old, new), b.sub(old, new)),
            (LessThan(i, a, b), _) => LessThan(i, a.sub(old, new), b.sub(old, new)),
            (GreaterThan(i, a, b), _) => GreaterThan(i, a.sub(old, new), b.sub(old, new)),
            (LessThanEqual(i, a, b), _) => LessThanEqual(i, a.sub(old, new), b.sub(old, new)),
            (GreaterThanEqual(i, a, b), _) => GreaterThanEqual(i, a.sub(old, new), b.sub(old, new)),
            (And(i, a, b), _) => And(i, a.sub(old, new), b.sub(old, new)),
            (Or(i, a, b), _) => Or(i, a.sub(old, new), b.sub(old, new)),
            (Not(i, a), _) => Not(i, a.sub(old, new)),
            (Function(i, x, f), _) => Function(i, x, f.sub(old, new)),
            (Define(i, a, b), _) => Define(i, a.sub(old, new), b.sub(old, new)),
        };

        Box::new(expr)
    }
}
