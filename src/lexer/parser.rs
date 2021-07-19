use super::{ast::Expression, token::Token, token::TokenType};
use crate::lexer::{lexer::Lexer, token::Object};
use TokenType::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code)]
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse(&mut self) -> Expression {
        self.expression()
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();
        println!("{:#?}", expr);

        while self.expect(vec![BANG_EQUAL, EQUAL_EQUAL]) {
            let op = self.previous();
            let cmp = self.comparison();
            expr = Expression::Binary(Box::new(expr), op, Box::new(cmp));
        }

        expr
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        while self.expect(vec![GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let op = self.previous();
            let right = self.term();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while self.expect(vec![MINUS, PLUS]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expression {
        if self.expect(vec![BANG, MINUS]) {
            let op = self.previous();
            let right = self.unary();
            return Expression::Unary(op, Box::new(right));
        }

        self.primary()
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        while self.expect(vec![SLASH, STAR]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn primary(&mut self) -> Expression {
        if self.expect(vec![FALSE]) {
            return Expression::Literal(Object::Bool(false));
        }

        if self.expect(vec![TRUE]) {
            return Expression::Literal(Object::Bool(true));
        }

        if self.expect(vec![NUMBER, STRING]) {
            return Expression::Literal(self.previous().literal);
        }

        if self.expect(vec![LEFT_PAREN]) {
            let expr = self.expression();
            self.consume(RIGHT_PAREN, "expect ')' after expression".to_string());
            return Expression::Grouping(Box::new(expr));
        }

        Expression::Mark
    }

    fn consume(&mut self, tag: TokenType, message: String) -> Token {
        if self.check(tag) {
            return self.advance();
        }

        panic!("{}", message)
    }

    fn expect(&mut self, tokens: Vec<TokenType>) -> bool {
        if tokens.iter().any(|t| self.check(*t)) {
            self.advance();
            return true;
        }

        false
    }

    fn check(&self, tag: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().tag == tag
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        };
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.current).unwrap().clone()
    }

    fn previous(&self) -> Token {
        self.tokens.get(self.current - 1).unwrap().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().tag == EOF
    }
}

#[test]
fn test() {
    // FIXME: Option Unwrap Error
    let mut l = Lexer::new(String::from("-3!=4 ="));
    l.scan_tokens();

    let mut parser = Parser::new(l.tokens);
    let exp = parser.parse();

    println!("{:#?}", exp);
}
