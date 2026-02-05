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
    for e in parsed.get_errors() {
        println!("[ERROR] start: {:?}, end: {:?}", e.range.start, e.range.end);
        println!("  * {:#?}", e.message);
        println!("");
    }

    // that boy will change name
    let mut boy = Box::new(User::new("Paul"));
    println!("{}", boy.get_name()); // print name in console
    boy.change_name_to("Mark");
    println!("{}", boy.get_name()); // and boy printing changed name

    // that girl never will change name
    let girl = Box::new(User::new("Martha"));
    println!("{}", girl.get_name()); // girl only can print name
}

struct User {
    name: String,
}

impl User {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: String::from(name.into()),
        }
    }
    pub fn change_name_to(&mut self, new_name: impl Into<String>) {
        self.name = new_name.into();
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
