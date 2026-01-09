mod ast;
mod atom;
mod parser;
pub use atom::{atom, Atom};

use crate::{
    lexer::{Keyword, Token},
    parser::Parser,
};
mod ir;
mod lexer;

fn main() {
    // Путь к NetherScript-файлу, который нужно распарсить
    let path = "./from/main.ns";
    let lexem = lexer::lexer(path);
    let mut parsed = Parser::new(lexem);
    println!(" ");
    println!(" ");
    println!(" ");
    println!(" ");
    println!("{:#?}", parsed.parse_fn_decl());
    println!("{:?}", parsed.position);
    println!(" ");
    println!(" ");
    println!("{:?}", parsed.tokens);
}
