use crate::ast::lexer::{Lexer, Token,TokenKind};
use crate::ast::{ASTExpression, ASTStatement};
pub struct Parser {
    tokens:Vec<Token>,
    current:usize,

}

impl Parser {
    pub fn new() -> Self{
        Self {tokens: Vec::new(),current: 0}
    }

    pub fn from_input(input:&str) ->Self{
        let mut lexer:Lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        while let Some(token)= lexer.next_token() {
            tokens.push(token);
        }
        Self {
            tokens,
            current:0
        }
    }

    pub fn from_tokens(tokens:Vec<Token>) -> Self {
        Self {
            tokens,
            current:0
        }
    }

    pub fn next_statement(&mut self) ->Option<ASTStatement> {

        return self.parse_statement();
    }
    fn parse_statement(&mut self) -> Option<ASTStatement> {
        let token = self.current().unwrap();
        let expr = self.parse_expression()?;
        return Some(ASTStatement::expression(expr))
    }

    fn parse_expression(&mut self) ->Option<ASTExpression> {
        let token = self.consume()?;
        return match token.kind {
            TokenKind::Number(number) => {
                Some(ASTExpression::number(number))
            }
            _ => {
                 None
            }
        }
    }

    fn peek(&self,offset:isize) ->Option<&Token> {
        self.tokens.get((self.current as isize +offset) as usize)
    }

    fn consume(&mut self) -> Option<&Token> {
        self.current += 1;
        let token =self.peek(-1)?;
        return Some(token);
    }
    fn current(&self) -> Option<&Token> {
        self.peek(0)
    }
}