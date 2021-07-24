use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

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

#[derive(Debug, Clone)]
pub enum UnionObject<'a> {
    /// 值类型
    Value(Object),
    /// 引用类型
    Reference(&'a Object),
}

impl From<String> for Object {
    fn from(v: String) -> Self {
        Object::String(v)
    }
}

impl From<f32> for Object {
    fn from(v: f32) -> Self {
        Object::Digit(v)
    }
}

impl From<bool> for Object {
    fn from(v: bool) -> Self {
        Object::Bool(v)
    }
}

impl Into<f32> for Object {
    fn into(self) -> f32 {
        match self {
            Object::Digit(v) => v,
            _ => todo!(),
        }
    }
}

impl<'a> From<Object> for Rc<UnionObject<'a>> {
    fn from(v: Object) -> Self {
        Rc::new(UnionObject::Value(v))
    }
}

impl<'a> From<&'a Object> for Rc<UnionObject<'a>> {
    fn from(v: &'a Object) -> Self {
        Rc::new(UnionObject::Reference(v))
    }
}

impl<'a> Into<Object> for Rc<UnionObject<'a>> {
    fn into(self) -> Object {
        match self.as_ref() {
            UnionObject::Value(v) => v.to_owned(),
            UnionObject::Reference(v) => todo!(),
        }
    }
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
    pub fn new<T: Display + AsRef<str>>(
        tag: TokenType,
        lexeme: T,
        literal: Object,
        line: usize,
    ) -> Self {
        Token {
            tag,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}
