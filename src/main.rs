mod nodes;
mod tokens;
mod lexer;
mod parser;
mod values;
mod eval;
mod operators;
mod repl;

fn main() {
    let mut repl = repl::Repl::new();
    repl.init();
}
