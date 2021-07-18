mod lexer;

use crate::lexer::lexer::Lexer;

use crate::lexer::token::TokenType::*;

fn main() {
    let mut l = Lexer::new(String::from("console<>+-*<=abc>= \"hello\"\n77"));

    l.scan_tokens(); 

    println!("{:?}", l.tokens);
}
