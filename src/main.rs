mod ast;
mod lexer;
mod parser;
mod token;

fn main() {
    // open the file
    let input = std::fs::read_to_string("syntax").unwrap();

    // create a lexer
    let mut lexer = lexer::Lexer::new(input);

    // read all tokens
    let tokens = lexer.read_all();

    // create a parser
    let mut parser = parser::Parser::new(tokens);

    // parse the tokens
    let ast = parser.parse();

    // print the AST
    println!("{:?}", ast);
}
