use crate::nodes::Expression;
use crate::values::Value;

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
            Ok(expression) => self.evaluate_atom(expression),
            Err(e) => {
                println!("Error: {:?}", e);
                vec![]
            }
        }
    }

    fn evaluate_atom(&mut self, expr: Expression) -> Vec<Value> {
        match expr {
            Expression::Number(_, dividend, divisor) => vec![Value::ComplexNumber(dividend, divisor, 0, 1)],
            _ => vec![],
        }
    }
}
