// http://www.craftinginterpreters.com/appendix-ii.html

use super::token::{Object, Token, TokenType};

#[allow(dead_code)]
#[derive(Debug, Clone)]
/// Expr
pub enum Expression {
    Literal(Object),
    Assignment(Token, Box<Expression>),
    Unary(Token, Box<Expression>),
    Binary(Box<Expression>, Token, Box<Expression>),
    Grouping(Box<Expression>),
    /// A and B, A or B
    Logical(Box<Expression>, Token, Box<Expression>),
    Call(Box<Expression>, Token, Vec<Expression>),
    Var(Token),
    Mark,
}

impl Expression {
    fn walk<F>(&self, visitor: &F) -> bool
    where
        F: Fn(&Expression) -> bool,
    {
        visitor(self)
            && match self {
                Expression::Literal(object) => {
                    println!("{:?}", object);
                    false
                }
                Expression::Assignment(token, exp) => {
                    println!("{:?}", token);
                    exp.walk(visitor)
                }
                Expression::Unary(_, _) => todo!(),
                Expression::Binary(_, _, _) => todo!(),
                Expression::Grouping(_) => todo!(),
                Expression::Logical(_, _, _) => todo!(),
                Expression::Mark => todo!(),
                Expression::Var(_) => todo!(),
                Expression::Call(callee, paren, arguments) => todo!(),
            }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    Print(Expression),
    Expression(Expression),
    Var(Token, Expression),
    Block(Vec<Statement>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    Function(Token, Vec<Token>, Vec<Statement>),
}

impl Statement {}

#[test]
fn expression_display() {
    let e = Expression::Assignment(
        Token::new(
            TokenType::IDENTIFIER,
            "name",
            Object::String("name".to_string()),
            1,
        ),
        Box::new(Expression::Literal(Object::Digit(1.0))),
    );

    let mut i = 0;
    e.walk(&|expr| {
        if i > 20 {
            return false;
        }
        // i += 1;
        true
    });
}
