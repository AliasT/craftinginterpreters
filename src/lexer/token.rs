use std::fmt::{Debug, Display};

use phf::phf_map;

#[allow(non_camel_case_types)]
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(String),
    Digit(f32),
    Bool(bool),

    // 占位符
    Placeholder,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            Object::String(v) => write!(f, "{}", v),
            Object::Digit(v) => write!(f, "{}", v),
            Object::Bool(v) => write!(f, "{}", v),
            Object::Placeholder => write!(f, ""),
        };

        Ok(())
    }
}

use TokenType::*;

#[allow(non_upper_case_globals)]
pub static Keywords: phf::Map<&'static str, TokenType> = phf_map! {
    "and"    => AND,
    "class"  => CLASS,
    "else"   => ELSE,
    "false"  => FALSE,
    "for"    => FOR,
    "fun"    => FUN,
    "if"     => IF,
    "nil"    => NIL,
    "or"     => OR,
    "print"  => PRINT,
    "return" => RETURN,
    "super"  => SUPER,
    "this"   => THIS,
    "true"   => TRUE,
    "var"    => VAR,
    "while"  => WHILE,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub tag: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
}

impl Token {
    pub fn new(tag: TokenType, lexeme: String, literal: Object, line: usize) -> Self {
        Token {
            tag,
            lexeme,
            literal,
            line,
        }
    }
}
