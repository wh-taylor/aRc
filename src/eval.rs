use crate::nodes::Expression;
use crate::values::Value;
use crate::values::gcd;

pub struct Evaluator {
    
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            
        }
    }

    pub fn evaluate(&mut self, code: String) -> Vec<Value> {
        let mut parser = crate::parser::Parser::new(code);
        let expression_result = parser.parse();
        match expression_result {
            Ok(expression) => self.evaluate_expression(expression),
            Err(e) => {
                println!("Error: {:?}", e);
                vec![]
            }
        }
    }

    fn evaluate_expression(&self, expr: Expression) -> Vec<Value> {
        let mut values = Vec::<Value>::new();
        match expr {
            Expression::Closure(_, x, f) => values.push(Value::Function(*x, *f)),
            Expression::Multiply(_, x, y) => values.extend(self.eval2(&multiply, *x, *y)),
            Expression::Add(_, x, y) => values.extend(self.eval2(&add, *x, *y)),
            Expression::Number(_, dividend, divisor) => values.push(Value::ComplexNumber(dividend, divisor, 0, 1)),
            Expression::ImaginaryConstant(_) => values.push(Value::ComplexNumber(0, 1, 1, 1)),
            Expression::Boolean(b) => values.push(Value::Boolean(b)),
            _ => {},
        }
        values
    }

    fn eval2(&self, f: &dyn Fn(Value, Value) -> Value, x_expr: Expression, y_expr: Expression) -> Vec<Value> {
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
