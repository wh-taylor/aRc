use std::collections::HashMap;
use crate::nodes::Expression;
use crate::nodes::ParseError;
use crate::operators::Operator;
use crate::values::Value;

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

    pub fn evaluate_expression(&mut self, expr: Expression) -> Result<Vec<Value>, Error> {
        let mut values = Vec::<Value>::new();
        match expr {
            Expression::Define(_, l, r) => {
                values.extend(self.define(l, r)?);
            },
            Expression::Function(_, x, f) => values.push(Value::Function(*x, *f)),
            Expression::Call(_, f, x) => values.extend(Operator::Call(*f, *x).get_values(self)?),
            Expression::Multiply(_, x, y) => values.extend(Operator::Multiply(*x, *y).get_values(self)?),
            Expression::Divide(_, x, y) => values.extend(Operator::Divide(*x, *y).get_values(self)?),
            Expression::PlusMinus(_, x) => {
                values.extend(Operator::Negate(*x.clone()).get_values(self)?);
                values.extend(self.evaluate_expression(*x)?);
            },
            Expression::Negate(_, x) => values.extend(Operator::Negate(*x).get_values(self)?),
            Expression::Add(_, x, y) => values.extend(Operator::Add(*x, *y).get_values(self)?),
            Expression::Subtract(_, x, y) => values.extend(Operator::Subtract(*x, *y).get_values(self)?),
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
            _ => {},
        }
        Ok(values)
    }
}
