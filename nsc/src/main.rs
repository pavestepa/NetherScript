use ns_lexer::lexer;
use ns_parser::Parser;
use ns_sema::analyze;

fn main() {
    // Путь к NetherScript-файлу, который нужно распарсить
    let path = "./from/main.ns";
    let lexem = lexer(path);
    let mut parsed = Parser::new(lexem);
    let module = parsed.parse_module();
    println!("{:#?}", module);

    match analyze(&module) {
        Ok(_ctx) => {
            println!("ns-sema: ok");
        }
        Err(diagnostics) => {
            eprintln!("ns-sema: {} error(s)", diagnostics.len());
            for d in diagnostics {
                eprintln!("{}", d.render_pretty());
            }
            std::process::exit(1);
        }
    }
}
