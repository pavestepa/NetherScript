mod er;
mod ir;
mod lexer;
mod middle_end;

fn main() {
    // Путь к TypeScript-файлу, который нужно распарсить
    let path = "./from/main.ts";
    let a = lexer::lexer(path);
}
