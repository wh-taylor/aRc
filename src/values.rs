use crate::nodes::Expression;
use gcd::Gcd;

#[derive(Clone, PartialEq)]
pub enum Value {
    ComplexNumber(isize, isize, isize, isize),
    Boolean(bool),
    Function(Expression, Expression),
    Tuple(Vec<Value>),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComplexNumber(a, b, c, d) => match (a, b, c, d) {
                (_, b, _, d) if *b == 0 && *d == 0               => write!(f, "ERROR: Real and imaginary divisors are zero"),
                (_, b, _, _) if *b == 0                          => write!(f, "ERROR: Real divisor is zero"),
                (_, _, _, d) if *d == 0                          => write!(f, "ERROR: Imaginary divisor is zero"),
                (a, b, c, d) if a % b == 0 && *c == 0 && *d != 0 => write!(f, "{}", a / b),
                (a, b, c, d) if *c == 0 && *d != 0               => write!(f, "{}/{}", a / gcd(*a, *b), b / gcd(*a, *b)),
                (a, b, c, d) if *a == 0 && *b != 0 && c == d     => write!(f, "i"),
                (a, b, c, d) if *a == 0 && *b != 0 && c % d == 0 => write!(f, "{}i", c / d),
                (a, b, c, d) if *a == 0 && *b != 0               => write!(f, "{}i/{}", c / gcd(*c, *d), d / gcd(*c, *d)),
                (a, b, c, d) if a % b == 0 && c == d             => write!(f, "{} + i", a / b),
                (a, b, c, d) if a % b == 0 && c % d == 0         => write!(f, "{} + {}i", a / b, c / d),
                (a, b, c, d) if c == d                           => write!(f, "{}/{} + i", a / gcd(*a, *b), b / gcd(*a, *b)),
                (a, b, c, d) if c % d == 0                       => write!(f, "{}/{} + {}i", a / gcd(*a, *b), b / gcd(*a, *b), c / d),
                (a, b, c, d) if a % b == 0                       => write!(f, "{} + {}i/{}", a / b, c / gcd(*c, *d), d / gcd(*c, *d)),
                (a, b, c, d)                                     => write!(f, "{}/{} + {}i/{}", a / gcd(*a, *b), b / gcd(*a, *b), c / gcd(*c, *d), d / gcd(*c, *d)),
            },
            Self::Boolean(true) => write!(f, "true"),
            Self::Boolean(false) => write!(f, "false"),
            Self::Function(a, b) => write!(f, "{} => {}", a, b),
            Self::Tuple(xs) => {
                write!(f, "(")?;
                let mut iter = xs.iter().peekable();
                while let Some(v) = iter.next() {
                    if iter.peek().is_none() {
                        write!(f, "{:?}", v)?;
                    } else {
                        write!(f, "{:?}, ", v)?;
                    }
                }
                write!(f, ")")
            },
        }
    }
}

pub fn gcd(x: isize, y: isize) -> isize {
    (x.abs() as usize).gcd(y.abs() as usize) as isize
        * (x / x.abs()) * (y / y.abs())
}
