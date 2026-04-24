use ns_lexer::lexer;
use ns_parser::Parser;

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
}
