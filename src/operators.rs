use crate::eval::Evaluator;
use crate::values::Value;
use crate::nodes::Expression;
use crate::eval::Error;

pub enum Operator {
    Call(Expression, Expression),
    Multiply(Expression, Expression),
    Divide(Expression, Expression),
    Negate(Expression),
    Add(Expression, Expression),
    Subtract(Expression, Expression),
}

impl Operator {
    pub fn get_values(&self, evaluator: &mut Evaluator) -> Result<Vec<Value>, Error> {
        match self {
            Operator::Call(x, y)     => evaluator.iterate_two_args(&builtin::call, x, y),
            Operator::Multiply(x, y) => evaluator.iterate_two_args(&builtin::multiply, x, y),
            Operator::Divide(x, y)   => evaluator.iterate_two_args(&builtin::divide, x, y),
            Operator::Negate(x)      => evaluator.iterate_one_arg(&builtin::negate, x),
            Operator::Add(x, y)      => evaluator.iterate_two_args(&builtin::add, x, y),
            Operator::Subtract(x, y) => evaluator.iterate_two_args(&builtin::subtract, x, y),
        }
    }
}

impl Evaluator {
    fn iterate_one_arg(&mut self, f: &dyn Fn(Value) -> Result<Vec<Value>, Error>, x_expr: &Expression) -> Result<Vec<Value>, Error> {
        let x_values = self.evaluate_expression(x_expr.clone())?;
        let mut values = Vec::<Value>::new();
        for x_value in x_values {
            values.extend(f(x_value)?);
        }
        values.dedup();
        Ok(values)
    }

    fn iterate_two_args(&mut self, f: &dyn Fn(Value, Value) -> Result<Vec<Value>, Error>, x_expr: &Expression, y_expr: &Expression) -> Result<Vec<Value>, Error> {
        let x_values = self.evaluate_expression(x_expr.clone())?;
        let y_values = self.evaluate_expression(y_expr.clone())?;
        let mut values = Vec::<Value>::new();
        for x_value in x_values {
            for y_value in y_values.clone() {
                values.extend(f(x_value.clone(), y_value)?);
            }
        }
        values.dedup();
        Ok(values)
    }
}

mod builtin {
    use crate::values::Value;
    use crate::values::gcd;
    use crate::eval::Error;

    pub fn call(x: Value, y: Value) -> Result<Vec<Value>, Error> {
        match (&x, &y) {
            (Value::ComplexNumber(_, _, _, _), Value::ComplexNumber(_, _, _, _)) => {
                multiply(x, y)
            },
            _ => Err(Error::MismatchedType),
        }
    }

    pub fn multiply(x: Value, y: Value) -> Result<Vec<Value>, Error> {
        match (x, y) {
            (Value::ComplexNumber(a1, b1, c1, d1), Value::ComplexNumber(a2, b2, c2, d2)) => {
                // Real Segment
                let af = a1 * a2 * d1 * d2 - c1 * c2 * b1 * b2;
                let bf = b1 * b2 * d1 * d2;
                let cf = a1 * c2 * b2 * d1 + a2 * c1 * b1 * d2;
                let df = bf;
                Ok(vec![Value::ComplexNumber(af, bf, cf, df)])
            },
            _ => Err(Error::MismatchedType),
        }
    }

    pub fn divide(x: Value, y: Value) -> Result<Vec<Value>, Error> {
        match (x, y) {
            (Value::ComplexNumber(a1, b1, c1, d1), Value::ComplexNumber(a2, b2, c2, d2)) => {
                // Real Segment
                let af = b2 * d2 * (a1 * a2 * d1 * d2 + c1 * c2 * b1 * b2);
                let bf = b1 * d1 * (a2 * a2 * d2 * d2 + c2 * c2 * b2 * b2);
                let cf = b2 * d2 * (a2 * c1 * b1 * d2 + a1 * c2 * d1 * b2);
                let df = bf;
                Ok(vec![Value::ComplexNumber(af, bf, cf, df)])
            },
            _ => Err(Error::MismatchedType),
        }
    }

    pub fn negate(x: Value) -> Result<Vec<Value>, Error> {
        match x {
            Value::ComplexNumber(a, b, c, d) => {
                Ok(vec![Value::ComplexNumber(-a, b, -c, d)])
            }
            _ => Err(Error::MismatchedType),
        }
    }

    pub fn add(x: Value, y: Value) -> Result<Vec<Value>, Error> {
        match (x, y) {
            (Value::ComplexNumber(a1, b1, c1, d1), Value::ComplexNumber(a2, b2, c2, d2)) => {
                // Real Segment
                let lcm = b1 * gcd(b1, b2) / b2;
                let bf = lcm;
                let af1 = a1 * (lcm / b1);
                let af2 = a2 * (lcm / b2);
                let af = af1 + af2;
                // Imaginary Segment
                let lcmi = d1 * gcd(d1, d2) / d2;
                let df = lcm;
                let cf1 = c1 * (lcmi / d1);
                let cf2 = c2 * (lcmi / d2);
                let cf = cf1 + cf2;
                Ok(vec![Value::ComplexNumber(af, bf, cf, df)])
            },
            _ => Err(Error::MismatchedType),
        }
    }

    pub fn subtract(x: Value, y: Value) -> Result<Vec<Value>, Error> {
        match (x, y) {
            (Value::ComplexNumber(a1, b1, c1, d1), Value::ComplexNumber(a2, b2, c2, d2)) => {
                // Real Segment
                let lcm = b1 * gcd(b1, b2) / b2;
                let bf = lcm;
                let af1 = a1 * (lcm / b1);
                let af2 = a2 * (lcm / b2);
                let af = af1 - af2;
                // Imaginary Segment
                let lcmi = d1 * gcd(d1, d2) / d2;
                let df = lcm;
                let cf1 = c1 * (lcmi / d1);
                let cf2 = c2 * (lcmi / d2);
                let cf = cf1 - cf2;
                Ok(vec![Value::ComplexNumber(af, bf, cf, df)])
            },
            _ => Err(Error::MismatchedType),
        }
    }
}