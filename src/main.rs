mod ast;
mod atom;
mod parser;
mod text_range;
pub use atom::{atom, Atom};
pub use text_range::TextRange;

use crate::{
    lexer::{Keyword, Token},
    parser::Parser,
};
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
    println!("{:#?}", parsed.parse_module());
    println!(" ");
}
