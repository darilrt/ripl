use crate::{ast::Ast, lexer::Lexer, parser::Parser};

pub fn parse(input: &str) -> Result<Ast, String> {
    let mut lexer = Lexer::new(input.to_owned());
    let tokens = lexer.read_all();

    let mut parser = Parser::new(tokens);

    Ok(parser.parse())
}
