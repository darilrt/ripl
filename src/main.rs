use crate::{interpreter::Interpreter, utils::parse};

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;
mod utils;

fn main() {
    let interpreter = Interpreter::new();

    interpreter.exec(parse("1 + 2 * 3").unwrap());
}
