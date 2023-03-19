use crate::eval::Evaluator;
use std::io;

pub struct Repl {
    evaluator: Evaluator,
}

impl Repl {
    pub fn new() -> Repl {
        let evaluator = Evaluator::new();
        Repl { evaluator }
    }

    pub fn init(&mut self) {
        println!("aRc, version 0.0.0");

        loop {
            let input = self.get_input();
            let result = self.evaluator.evaluate(input);
            match result {
                Ok(values) => {
                    let mut values_iter = values.iter().peekable();
                    while let Some(value) = values_iter.next() {
                        if values_iter.peek().is_none() {
                            println!("{:?}\n", value);
                        } else {
                            print!("{:?}, ", value);
                        }
                    }
                },
                Err(_) => {},
            }
        }
    }

    fn get_input(&self) -> String {
        let mut buffer = String::new();
        print!("> ");
        
        let _ = io::Write::flush(&mut io::stdout());
        
        io::stdin().read_line(&mut buffer).expect("Input Error");
        
        if let Some('\n') = buffer.chars().next_back() {
            buffer.pop();
        }
        if let Some('\r') = buffer.chars().next_back() {
            buffer.pop();
        }
        
        buffer
    }
}