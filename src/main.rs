mod ast;
use ast::lexer::{*};
use crate::ast::{parser::Parser, Ast};
fn main() {
    let input = "77+543 * 512332 rfef";
    let mut lexer = ast::lexer::Lexer::new(input);
    let mut tokens:Vec<Token> = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token)
    }
    println!("{:?}",tokens);

    let mut ast = Ast::new();
    let mut parser = Parser::from_input("77");
    while let Some(stmt) = parser.next_statement() {
        ast.add_statement(stmt);
    }
    ast.visualize();
}