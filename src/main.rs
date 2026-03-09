pub mod semantics;
pub mod syntax;
pub mod utils;
pub use utils::{atom, Atom};

use crate::syntax::lexer::lexer;
use crate::syntax::parser::Parser;
fn main() {
    // Путь к NetherScript-файлу, который нужно распарсить
    let path = "./from/main.ns";
    let lexem = lexer(path);
    let mut parsed = Parser::new(lexem);
    println!(" ");
    println!(" ");
    println!(" ");
    println!(" ");
    println!("{:#?}", parsed.parse_module());
    println!(" ");
    for e in parsed.get_errors() {
        println!("[ERROR] start: {:?}, end: {:?}", e.range.start, e.range.end);
        println!("  * {:#?}", e.message);
        println!("");
    }
}
