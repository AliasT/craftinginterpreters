use std::{
    any::{Any, TypeId},
    convert::TryInto,
};

use super::ast::Expression;
use crate::lexer::{lexer::Lexer, parser::Parser, token::Object, token::TokenType};
use TokenType::*;

pub struct Compiler {
    // pub expr: Expression,
}

impl Compiler {
    fn new() -> Self {
        Compiler {}
    }

    fn compile_expr(&self, expr: Expression) -> Object {
        match expr {
            Expression::Literal(v) => match v {
                v => v,
            },
            Expression::Unary(token, ex) => {
                let ret = self.compile_expr(*ex);
                if let Object::Digit(n) = ret {
                    if token.tag == MINUS {
                        return Object::Digit(-n);
                    }
                }

                if let Object::Bool(n) = ret {
                    if token.tag == BANG {
                        return Object::Bool(!n);
                    }
                }

                panic!("expected expression")
            }
            Expression::Assignment(_, _) => todo!(),
            Expression::Binary(le, op, re) => {
                let left = self.compile_expr(*le);
                let right = self.compile_expr(*re);

                if let (Object::Digit(lv), Object::Digit(rv)) = (left, right) {
                    return match op.tag {
                        PLUS => Object::Digit(lv + rv),
                        MINUS => Object::Digit(lv - rv),
                        SLASH => Object::Digit(lv / rv),
                        STAR => Object::Digit(lv * rv),
                        GREATER => Object::Bool(lv > rv),
                        GREATER_EQUAL => Object::Bool(lv >= rv),
                        LESS => Object::Bool(lv < rv),
                        LESS_EQUAL => Object::Bool(lv <= rv),

                        // TODO：string 的比较
                        BANG_EQUAL => Object::Bool(lv != rv),
                        EQUAL => Object::Bool(lv == rv),
                        _ => {
                            panic!("expected expression")
                        }
                    };
                }

                panic!("expected expression")
            }
            Expression::Grouping(ex) => self.compile_expr(*ex),
            Expression::Logical(_, _, _) => todo!(),
            Expression::Mark => todo!(),
        }
    }
}
#[test]
fn test() {
    // FIXME: Option Unwrap Error
    let mut l = Lexer::new(String::from("1+6/(3+3)*2"));
    l.scan_tokens();

    let mut parser = Parser::new(l.tokens);
    let expr = parser.parse();

    let result = Compiler::new().compile_expr(expr);
    println!("{:?}", result);
}
