pub mod lexer;
pub mod parser;

pub struct Ast {
    pub statements:Vec<ASTStatement>
}
pub enum ASTExpressionKind {
    Number(
        ASTNumberExpression
    ),
    Binary(
        ASTBinaryExpression
    )
}

pub enum ASTBinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide
}

pub struct ASTBinaryExpression {
    left:Box<ASTExpression>,
    operator:ASTBinaryOperator,
    right:Box<ASTExpression>,
}

pub struct ASTNumberExpression {
    number:i64,
}
pub struct ASTExpression {
    kind: ASTExpressionKind
}
pub enum ASTStatementKind {
    expression(ASTExpression),
}
pub struct ASTStatement {
    kind:ASTStatementKind
}

impl Ast {
    pub fn new() -> Self  {
        Self {statements:Vec::new()}
    }

    pub fn add_statement(&mut self, statement:ASTStatement) {
        self.statements.push(statement);
    }

    pub fn visit(&mut self,visitor: &mut dyn ASTVisitor) {
        for statement in &self.statements {
            visitor.visit_statement(statement);
        }
    }

    pub fn visualize(&mut self)  {
        let mut printer = ASTPrinter{indent:0};
        self.visit(&mut printer);
    }
}
pub trait ASTVisitor {
    fn do_visit_statement(&mut self,statement:&ASTStatement)
    {
        match &statement.kind {
            ASTStatementKind::expression(expr) => {
                self.visit_expression(expr)
            }
        }
    }

    fn visit_statement(&mut self,statement:&ASTStatement)
    {
        self.do_visit_statement(statement);
    }
    fn do_visit_expression(&mut self,expression:&ASTExpression)
    {
        match &expression.kind {
            ASTExpressionKind::Number(number) => {
                self.visit_number(number);
            }
        }
    }

    fn visit_expression(&mut self,expression:&ASTExpression) 
    {
       self.do_visit_expression(expression);
    }

    fn visit_number(&mut self,number:&ASTNumberExpression);
}

pub struct ASTPrinter{
    indent:usize,
}
const LEVEL_INDENT:usize = 2;
impl ASTVisitor for ASTPrinter {
    fn visit_number(&mut self,numberexpr:&ASTNumberExpression) {
        println!("{}{}"," ".repeat(self.indent),numberexpr.number);
    }

    fn visit_statement(&mut self,statement:&ASTStatement) {
        self.print_with_indent("Statement:");
        self.indent += LEVEL_INDENT;
        ASTVisitor::do_visit_statement(self, statement);
        self.indent -= LEVEL_INDENT;

    }

    fn visit_expression(&mut self,expression:&ASTExpression) {
        self.print_with_indent("Expression:");
        self.indent += LEVEL_INDENT;
        ASTVisitor::do_visit_expression(self, expression);
        self.indent -= LEVEL_INDENT;
    }
}

impl ASTPrinter {
    fn print_with_indent(&mut self,text:&str) {
        println!("{}{}"," ".repeat(self.indent),text);
    }
}

impl ASTStatement{
    pub fn new(kind: ASTStatementKind) ->Self {
        ASTStatement{ kind}
    }

    pub fn expression(expr:ASTExpression) ->Self {
        ASTStatement::new(ASTStatementKind::expression(expr))
    }

}

impl ASTExpression {
    pub fn new(kind:ASTExpressionKind) -> Self {
        ASTExpression {kind}
    }

    pub fn number(number:i64) -> Self {
        ASTExpression::new(ASTExpressionKind::Number(ASTNumberExpression { number: number }))
    }

}


