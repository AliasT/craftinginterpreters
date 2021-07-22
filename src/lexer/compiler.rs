use super::{
    ast::{Expression, Statement},
    token::UnionObject,
    vm::VM,
};
use crate::lexer::{lexer::Lexer, parser::Parser, token::Object, token::TokenType};
use TokenType::*;

#[derive(Debug)]
pub struct Compiler<'a> {
    // pub expr: Expression,
    vm: VM<'a>,
}

impl<'a> Compiler<'a> {
    fn new() -> Self {
        Compiler { vm: VM::new() }
    }

    fn interpret(&mut self, statements: Vec<Statement>) {
        for stmt in statements {
            self.compile_stmt(stmt);
        }
    }

    fn compile_expr(&mut self, expr: Expression) -> UnionObject<'a> {
        match expr {
            Expression::Literal(v) => match v {
                v => v.into(),
            },
            Expression::Unary(token, ex) => {
                let ret = self.compile_expr(*ex);
                if let UnionObject::Value(Object::Digit(n)) = ret {
                    if token.tag == MINUS {
                        return Object::from(-n).into();
                    }
                }

                if let UnionObject::Value(Object::Bool(n)) = ret {
                    if token.tag == BANG {
                        return Object::from(!n).into();
                    }
                }

                panic!("expected expression")
            }
            Expression::Assignment(ident, exp) => {
                let value = self.compile_expr(*exp);
                let t = value.clone();
                self.vm.assign(ident.lexeme, value);
                t
            }
            Expression::Binary(le, op, re) => {
                let left = self.compile_expr(*le).into();
                let right = self.compile_expr(*re).into();

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
                    }
                    .into();
                }

                panic!("expected expression")
            }
            Expression::Grouping(ex) => self.compile_expr(*ex),
            Expression::Logical(_, _, _) => todo!(),
            Expression::Mark => todo!(),
            Expression::Var(token) => self.vm.retrieve(token.lexeme).clone(),
        }
    }

    fn compile_stmt(&mut self, stmt: Statement) {
        match stmt {
            Statement::Expression(expr) => {
                self.compile_expr(expr);
            }
            Statement::Print(expr) => {
                let value = self.compile_expr(expr);
                println!("{}", <UnionObject<'a> as Into<Object>>::into(value))
            }
            Statement::Var(name, initializer) => {
                let value = self.compile_expr(initializer);
                // FIXME: unwrap Object
                self.vm.define(name.lexeme, value);
            }
        };
    }
}

#[test]
fn test() {
    // FIXME: Option Unwrap Error
    let mut l = Lexer::new(String::from("var a = 3\nvar b = a\nprint b"));
    l.scan_tokens();

    let mut parser = Parser::new(l.tokens);
    let statements = parser.parse();

    let _ = Compiler::new().interpret(statements);
    // assert_eq!(result, Object::Digit(3.0));
}
