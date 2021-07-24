use crate::lexer::token::Keywords;

use super::token::{Object, Token, TokenType};
use TokenType::*;

pub struct Lexer {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

#[allow(dead_code)]
impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            source: input.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.get(self.current).unwrap();
        self.current += 1;
        *c
    }

    /// 获取指定索引 index 的 char
    pub fn get_ch(&mut self, index: usize) -> char {
        *self.source.get(index).unwrap()
    }

    pub fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        };
        self.get_ch(self.current)
    }

    pub fn peek_next(&mut self) -> char {
        if self.current + 1 > self.source.len() {
            return '\0';
        };
        self.get_ch(self.current + 1)
    }

    pub fn get_by_range(&self, start: usize, end: usize) -> String {
        self.source.get(start..end).unwrap().iter().collect()
    }

    fn add_token(&mut self, tag: TokenType, literal: Object) {
        let text: String = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .iter()
            .collect();
        self.tokens.push(Token::new(tag, text, literal, self.line));
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        use self::*;
        match c {
            '(' => self.add_token(LEFT_PAREN, Object::Placeholder),
            ')' => self.add_token(RIGHT_PAREN, Object::Placeholder),
            '{' => self.add_token(LEFT_BRACE, Object::Placeholder),
            '}' => self.add_token(RIGHT_BRACE, Object::Placeholder),
            ',' => self.add_token(COMMA, Object::Placeholder),
            '.' => self.add_token(DOT, Object::Placeholder),
            '-' => self.add_token(MINUS, Object::Placeholder),
            '+' => self.add_token(PLUS, Object::Placeholder),
            ';' => self.add_token(SEMICOLON, Object::Placeholder),
            '*' => self.add_token(STAR, Object::Placeholder),
            '!' => {
                let t = if self.expect('=') { BANG_EQUAL } else { BANG };
                self.add_token(t, Object::Placeholder)
            }
            '=' => {
                let t = if self.expect('=') { EQUAL_EQUAL } else { EQUAL };
                self.add_token(t, Object::Placeholder)
            }
            '<' => {
                let t = if self.expect('=') { LESS_EQUAL } else { LESS };
                self.add_token(t, Object::Placeholder)
            }
            '>' => {
                let t = if self.expect('=') {
                    GREATER_EQUAL
                } else {
                    GREATER
                };
                self.add_token(t, Object::Placeholder)
            }
            '/' => {
                if self.expect('/') {
                    // 本行是注释，扫描到行结尾
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(SLASH, Object::Placeholder);
                }
            }
            '"' => {
                while self.peek() != '"' && !self.is_at_end() {
                    if self.peek() == '\n' {
                        self.line += 1
                    }
                    self.advance();
                }

                if self.is_at_end() {
                    panic!("unterminated string");
                }

                self.advance();
                let value = self.get_by_range(self.start + 1, self.current - 1);

                self.add_token(STRING, Object::String(value));
            }
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => { /* ignore white space */ }
            c => {
                if c.is_numeric() {
                    // 一直到不为数字为止
                    while self.peek().is_numeric() {
                        self.advance();
                    }

                    // 尝试寻找浮点数字
                    if self.peek() == '.' && self.peek_next().is_numeric() {
                        // 跳过小数点
                        self.advance();
                        while self.peek().is_numeric() {
                            self.advance();
                        }
                    }

                    let value = self
                        .get_by_range(self.start, self.current)
                        .parse::<f32>()
                        .expect("解析数字出错");

                    self.add_token(NUMBER, Object::Digit(value));
                } else if c.is_ascii_alphanumeric() {
                    // 忽略下划线的关键字
                    while self.peek().is_ascii_alphanumeric() {
                        self.advance();
                    }
                    let value = self.get_by_range(self.start, self.current);
                    if let Some(tag) = Keywords.get(value.as_str()) {
                        self.add_token(*tag, Object::Placeholder);
                    } else {
                        self.add_token(IDENTIFIER, Object::Placeholder);
                    }
                } else {
                    println!("unexpected token: {}", c)
                }
            }
        }
    }

    ///
    fn expect(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.get_ch(self.current) != expected {
            return false;
        }

        self.current += 1;

        return true;
    }
}
