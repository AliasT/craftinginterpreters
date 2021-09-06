// http://www.craftinginterpreters.com/appendix-i.html

use std::{borrow::Borrow, cell::RefCell, collections::btree_set::Union, env, rc::Rc};

use crate::lang::environment;

use super::{
    ast::{Expression, Statement},
    environment::Environment,
    token::{Object, TokenType::*, UnionObject},
};

#[derive(Debug)]
pub struct Compiler<'a> {
    // pub expr: Expression,
    environment: Rc<RefCell<Environment<'a>>>,
}

#[allow(dead_code)]
impl<'a> Compiler<'a> {
    fn new() -> Self {
        Compiler {
            environment: Rc::new(RefCell::new(Environment::new(
                Option::<Rc<RefCell<Environment>>>::None,
            ))),
        }
    }

    fn interpret(&'a mut self, statements: Vec<Statement>) {
        for stmt in statements {
            self.compile_stmt(stmt);
        }
    }

    fn compile_expr(&mut self, expr: Expression) -> Rc<UnionObject<'a>> {
        match expr {
            Expression::Literal(v) => match v {
                v => v.into(),
            },
            Expression::Unary(token, ex) => {
                let ret = self.compile_expr(*ex);
                if let UnionObject::Value(Object::Digit(n)) = ret.as_ref() {
                    if token.tag == MINUS {
                        return Object::from(-n).into();
                    }
                }

                if let UnionObject::Value(Object::Bool(n)) = ret.as_ref() {
                    if token.tag == BANG {
                        return Object::from(!n).into();
                    }
                }

                panic!("expected expression")
            }
            Expression::Assignment(ident, exp) => {
                let value = self.compile_expr(*exp);
                self.environment
                    .borrow_mut()
                    .assign(ident.lexeme, value.clone());
                value
            }
            Expression::Binary(le, op, re) => {
                let left = self.compile_expr(*le).into();
                let right = self.compile_expr(*re).into();

                if let (Object::Digit(lv), Object::Digit(rv)) = (left, right) {
                    return match op.tag {
                        // TODO: 使用 Operator overload
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
            Expression::Var(token) => self.environment.borrow_mut().retrieve(token.lexeme).clone(),
            Expression::Call(callee, _, arguments) => {
                if let UnionObject::Function(function) = self.compile_expr(*callee).borrow() {
                    if let Statement::Function(a, parameters, stmts) = function.borrow() {
                        // 参数计算
                        let mut args = Vec::<Rc<UnionObject>>::new();
                        for expr in arguments {
                            args.push(self.compile_expr(expr))
                        }

                        // 补充上下文
                        let mut environment = Environment::new(self.environment.clone());
                        for (i, name) in parameters.iter().enumerate() {
                            environment.define(name.lexeme.clone(), args[i].clone());
                        }
                        let prev = self.environment.clone();
                        self.environment = Rc::new(RefCell::new(environment));
                        for stmt in stmts {
                            self.compile_stmt(stmt.to_owned());
                        }
                        self.environment = prev;
                    }
                }
                // TODO: return statement
                Object::Bool(false).into()
            }
        }
    }

    // fn call_func(&mut self, )

    fn compile_stmt(&mut self, stmt: Statement) {
        match stmt {
            Statement::Expression(expr) => {
                self.compile_expr(expr);
            }
            Statement::Print(expr) => {
                let value = self.compile_expr(expr);
                println!("{}", <Rc<UnionObject<'a>> as Into<Object>>::into(value))
            }
            Statement::Var(name, initializer) => {
                let value = self.compile_expr(initializer);
                self.environment.borrow_mut().define(name.lexeme, value);
            }
            Statement::Block(statements) => {
                let previous = self.environment.clone();
                let inner = Environment::new(self.environment.clone());
                self.environment = Rc::new(RefCell::new(inner));
                for stmt in statements {
                    self.compile_stmt(stmt);
                }

                self.environment = previous;
            }
            Statement::If(condition, then_stmt, else_stmt) => {
                if let Object::Bool(truty) = self.compile_expr(condition).into() {
                    if truty {
                        self.compile_stmt(*then_stmt)
                    } else if let Some(else_stmt) = else_stmt {
                        self.compile_stmt(*else_stmt)
                    }
                } else {
                    panic!("expect a bool within if statement")
                }
            }
            function => {
                if let Statement::Function(name, ..) = &function {
                    self.environment.borrow_mut().define(
                        (*name).lexeme.clone(),
                        Rc::new(UnionObject::Function(Rc::new(function))),
                    );
                }
            }
        };
    }
}

#[test]
fn test() {
    use super::{lexer::Lexer, parser::Parser};
    // FIXME: Option Unwrap Error
    let mut l = Lexer::new(String::from(
        "function a(){
            print 1
        }

        a()

        ",
    ));
    l.scan_tokens();

    let mut parser = Parser::new(l.tokens);
    let statements = parser.parse();
    println!("{:?}", statements);

    let _ = Compiler::new().interpret(statements);
    // assert_eq!(result, Object::Digit(3.0));
}
