mod ast;
mod parser;
mod atom;
pub use atom::{atom, Atom};
mod ir;
mod lexer;

fn main() {
    // Путь к TypeScript-файлу, который нужно распарсить
    let path = "./from/main.ns";
    let a = lexer::lexer(path);
    let b = parser::parse_module(a);
    println!("{:?}", b);
}
