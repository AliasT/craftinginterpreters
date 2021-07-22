use std::os::macos::raw::stat;

use super::{
    ast::{Expression, Statement},
    vm::VM,
};
use crate::lexer::{lexer::Lexer, parser::Parser, token::Object, token::TokenType};
use TokenType::*;

#[derive(Debug)]
pub struct Compiler {
    // pub expr: Expression,
    vm: VM,
}

impl Compiler {
    fn new() -> Self {
        Compiler { vm: VM::new() }
    }

    fn interpret(&mut self, statements: Vec<Statement>) {
        for stmt in statements {
            self.compile_stmt(stmt);
        }
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
            Expression::Var(_) => todo!(),
        }
    }

    fn compile_stmt(&mut self, stmt: Statement) {
        match stmt {
            Statement::Expression(expr) => {
                self.compile_expr(expr);
            }
            Statement::Print(expr) => {
                let value = self.compile_expr(expr);
                println!("{}", value)
            }
            Statement::Var(name, initializer) => {
                let value = self.compile_expr(initializer);
                // FIXME: unwrap Object
                self.vm.define(name.lexeme, value);
                println!("{:?}", self.vm);
            }
        };
    }
}

#[test]
fn test() {
    // FIXME: Option Unwrap Error
    let mut l = Lexer::new(String::from("var a = 3"));
    l.scan_tokens();

    let mut parser = Parser::new(l.tokens);
    let statements = parser.parse();

    let _ = Compiler::new().interpret(statements);
    // assert_eq!(result, Object::Digit(3.0));
}
