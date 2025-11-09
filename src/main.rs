mod ast;
mod ast_parser;
mod atom;
pub use atom::{atom, Atom};
mod ir;
mod lexer;
mod middle_end;

fn main() {
    // Путь к TypeScript-файлу, который нужно распарсить
    let path = "./from/main.ns";
    let a = lexer::lexer(path);
    let b = ast_parser::parse_module(a);
    println!("{:?}", b);
}
