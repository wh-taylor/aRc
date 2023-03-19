use std::collections::HashMap;
use crate::nodes::Expression;
use crate::nodes::ParseError;
use crate::values::Value;
use crate::values::gcd;

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    MismatchedType,
}

pub struct Evaluator {
    definitions: Vec<HashMap<String, Vec<Value>>>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            definitions: vec![HashMap::new()],
        }
    }

    pub fn evaluate(&mut self, code: String) -> Result<Vec<Value>, Error> {
        let mut parser = crate::parser::Parser::new(code);
        let values;
        let expression_result = parser.parse();
        match expression_result {
            Ok(expression) => values = self.evaluate_expression(expression),
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(Error::ParseError(e));
            }
        }
        values
    }

    fn evaluate_expression(&mut self, expr: Expression) -> Result<Vec<Value>, Error> {
        let mut values = Vec::<Value>::new();
        match expr {
            Expression::Define(_, l, r) => {
                values.extend(self.define(l, r)?);
            },
            Expression::Function(_, x, f) => values.push(Value::Function(*x, *f)),
            Expression::Call(_, f, x) => values.extend(self.call(*f, *x)?),
            Expression::Multiply(_, x, y) => values.extend(self.eval2(&multiply, *x, *y)?),
            Expression::Divide(_, x, y) => values.extend(self.eval2(&divide, *x, *y)?),
            Expression::PlusMinus(_, x) => {
                values.extend(self.eval1(&negate, *x.clone())?);
                values.extend(self.eval1(&|x| Ok(vec![x]), *x)?);
            },
            Expression::Negate(_, x) => values.extend(self.eval1(&negate, *x)?),
            Expression::Add(_, x, y) => values.extend(self.eval2(&add, *x, *y)?),
            Expression::Subtract(_, x, y) => values.extend(self.eval2(&subtract, *x, *y)?),
            Expression::Number(_, dividend, divisor) => values.push(Value::ComplexNumber(dividend, divisor, 0, 1)),
            Expression::ImaginaryConstant(_) => values.push(Value::ComplexNumber(0, 1, 1, 1)),
            Expression::Variable(_, name) => {
                let result = self.get_definition(name);
                match result {
                    Some(value) => values.extend(value),
                    None => {},
                }
            },
            Expression::Boolean(b) => values.push(Value::Boolean(b)),
            Expression::Tuple(_, xs) => {
                let mut tuple_values = Vec::new();
                for expr in xs {
                    let value = self.evaluate_expression(expr)?;
                    tuple_values.extend(value);
                }
                values.push(Value::Tuple(tuple_values))
            },
            _ => {},
        }
        Ok(values)
    }

    fn get_definition(&self, name: String) -> Option<Vec<Value>> {
        for scope in self.definitions.iter().rev() {
            match scope.get(&name) {
                Some(x) => return Some(x.clone()),
                None => continue,
            }
        }
        None
    }

    fn _increase_scope(&mut self) {
        self.definitions.push(HashMap::new());
    }

    fn _decrease_scope(&mut self) {
        self.definitions.pop();
    }

    fn define(&mut self, l: Box<Expression>, r: Box<Expression>) -> Result<Vec<Value>, Error> {
        let mut values = Vec::<Value>::new();
        match *l {
            Expression::Variable(_, name) => {
                let value = self.evaluate_expression(*r.clone())?;
                self.definitions.last_mut().unwrap().insert(name, value);
                values.extend(self.evaluate_expression(*r)?);
            },
            Expression::Call(i, f, x) => {
                let closure = Expression::Function(i, x.clone(), r);
                values.extend(self.define(f, Box::new(closure))?);
            },
            _ => {},
        }
        Ok(values)
    }

    fn call(&mut self, x: Expression, y: Expression) -> Result<Vec<Value>, Error> {
        let x_values = self.evaluate_expression(x.clone())?;
        match (&x_values[..], y.clone()) {
            ([Value::Function(old, expr)], _) => {
                let new_expr = expr.sub(&old, &y);
                self.evaluate_expression(*new_expr)
            },
            _ => self.eval2(&multiply, x, y),
        }
    }

    fn eval1(&mut self, f: &dyn Fn(Value) -> Result<Vec<Value>, Error>, x_expr: Expression) -> Result<Vec<Value>, Error> {
        let x_values = self.evaluate_expression(x_expr)?;
        let mut values = Vec::<Value>::new();
        for x_value in x_values {
            values.extend(f(x_value)?);
        }
        values.dedup();
        Ok(values)
    }

    fn eval2(&mut self, f: &dyn Fn(Value, Value) -> Result<Vec<Value>, Error>, x_expr: Expression, y_expr: Expression) -> Result<Vec<Value>, Error> {
        let x_values = self.evaluate_expression(x_expr)?;
        let y_values = self.evaluate_expression(y_expr)?;
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

fn multiply(x: Value, y: Value) -> Result<Vec<Value>, Error> {
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

fn divide(x: Value, y: Value) -> Result<Vec<Value>, Error> {
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

fn negate(x: Value) -> Result<Vec<Value>, Error> {
    match x {
        Value::ComplexNumber(a, b, c, d) => {
            Ok(vec![Value::ComplexNumber(-a, b, -c, d)])
        }
        _ => Err(Error::MismatchedType),
    }
}

fn add(x: Value, y: Value) -> Result<Vec<Value>, Error> {
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

fn subtract(x: Value, y: Value) -> Result<Vec<Value>, Error> {
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
