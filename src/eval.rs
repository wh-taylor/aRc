use std::collections::HashMap;
use crate::nodes::Expression;
use crate::values::Value;
use crate::values::gcd;

pub struct Evaluator {
    definitions: Vec<HashMap<String, Expression>>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            definitions: vec![HashMap::new()],
        }
    }

    pub fn evaluate(&mut self, code: String) -> Vec<Value> {
        self.increase_scope();
        let mut parser = crate::parser::Parser::new(code);
        let values;
        let expression_result = parser.parse();
        match expression_result {
            Ok(expression) => values = self.evaluate_expression(expression),
            Err(e) => {
                println!("Error: {:?}", e);
                return vec![];
            }
        }
        self.decrease_scope();
        values
    }

    fn evaluate_expression(&mut self, expr: Expression) -> Vec<Value> {
        let mut values = Vec::<Value>::new();
        match expr {
            Expression::Define(_, l, r) => {
                if let Expression::Variable(_, name) = *l {
                    self.definitions.first_mut().unwrap().insert(name, *r.clone());
                    values.extend(self.evaluate_expression(*r));
                }
            },
            Expression::Function(_, f, x) => {
                let function = self.evaluate_expression(*f.clone())[0].clone();
                match function {
                    Value::Function(input, closure) => {
                        if let Expression::Variable(_, name) = input {
                            self.define(name, *x);
                            values.extend(self.evaluate_expression(closure));
                        }
                    },
                    _ => values.extend(self.eval2(&multiply, *f, *x)),
                }
            },
            
            Expression::Closure(_, x, f) => values.push(Value::Function(*x, *f)),
            Expression::Multiply(_, x, y) => values.extend(self.eval2(&multiply, *x, *y)),
            Expression::PlusMinus(_, x) => {
                values.extend(self.eval1(&negate, *x.clone()));
                values.extend(self.eval1(&|x| x, *x));
            },
            Expression::Negate(_, x) => values.extend(self.eval1(&negate, *x)),
            Expression::Add(_, x, y) => values.extend(self.eval2(&add, *x, *y)),
            Expression::Subtract(_, x, y) => values.extend(self.eval2(&subtract, *x, *y)),
            Expression::Number(_, dividend, divisor) => values.push(Value::ComplexNumber(dividend, divisor, 0, 1)),
            Expression::ImaginaryConstant(_) => values.push(Value::ComplexNumber(0, 1, 1, 1)),
            Expression::Variable(_, name) => {
                let result = self.get_definition(name);
                match result {
                    Some(expr) => values.extend(self.evaluate_expression(expr.clone())),
                    None => {},
                }
            },
            Expression::Boolean(b) => values.push(Value::Boolean(b)),
            _ => {},
        }
        values
    }

    fn get_definition(&self, name: String) -> Option<Expression> {
        for scope in self.definitions.iter().rev() {
            match scope.get(&name) {
                Some(x) => return Some(x.clone()),
                None => continue,
            }
        }
        None
    }

    fn increase_scope(&mut self) {
        self.definitions.push(HashMap::new());
    }

    fn decrease_scope(&mut self) {
        self.definitions.pop();
    }

    fn define(&mut self, name: String, expr: Expression) -> Vec<Value> {
        match self.definitions.last_mut() {
            Some(map) => {map.insert(name, expr.clone());},
            None => {},
        }
        self.evaluate_expression(expr)
    }

    fn eval1(&mut self, f: &dyn Fn(Value) -> Value, x_expr: Expression) -> Vec<Value> {
        let x_values = self.evaluate_expression(x_expr);
        let mut values = Vec::<Value>::new();
        for x_value in x_values {
            values.push(f(x_value));
        }
        values
    }

    fn eval2(&mut self, f: &dyn Fn(Value, Value) -> Value, x_expr: Expression, y_expr: Expression) -> Vec<Value> {
        let x_values = self.evaluate_expression(x_expr);
        let y_values = self.evaluate_expression(y_expr);
        let mut values = Vec::<Value>::new();
        for x_value in x_values {
            for y_value in y_values.clone() {
                values.push(f(x_value.clone(), y_value));
            }
        }
        values
    }
}

fn multiply(x: Value, y: Value) -> Value {
    match (x, y) {
        (Value::ComplexNumber(a1, b1, c1, d1), Value::ComplexNumber(a2, b2, c2, d2)) => {
            // Real Segment
            let af = a1 * a2 * d1 * d2 - c1 * c2 * b1 * b2;
            let bf = b1 * b2 * d1 * d2;
            let cf = a1 * c2 * b2 * d1 + a2 * c1 * b1 * d2;
            let df = bf;
            Value::ComplexNumber(af, bf, cf, df)
        },
        _ => Value::ComplexNumber(0, 1, 0, 1),
    }
}

fn negate(x: Value) -> Value {
    match x {
        Value::ComplexNumber(a, b, c, d) => {
            Value::ComplexNumber(-a, b, -c, d)
        }
        _ => Value::ComplexNumber(0, 1, 0, 1),
    }
}

fn add(x: Value, y: Value) -> Value {
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
            Value::ComplexNumber(af, bf, cf, df)
        },
        _ => Value::ComplexNumber(0, 1, 0, 1),
    }
}

fn subtract(x: Value, y: Value) -> Value {
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
            Value::ComplexNumber(af, bf, cf, df)
        },
        _ => Value::ComplexNumber(0, 1, 0, 1),
    }
}
