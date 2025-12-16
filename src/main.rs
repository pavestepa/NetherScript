mod ast;
mod parser;
mod atom;
pub use atom::{atom, Atom};

use crate::{lexer::{Keyword, Token}, parser::Parser};
mod ir;
mod lexer;

fn main() {
    // Путь к NetherScript-файлу, который нужно распарсить
    let path = "./from/main.ns";
    let lexem = lexer::lexer(path);
    println!(" ");
    println!(" ");
    let mut parsed = Parser::new(lexem);
    println!("{:?}", parsed.tokens);
    println!(" ");
    println!(" ");
    println!("{:?}", parsed.first_finded(Token::GreaterThanOrEqual));
    println!("{:?}", parsed.position);
    println!(" ");
    println!(" ");
    println!("{:?}", parsed.tokens);
}
