use ns_parser::main_parse_with_diagnostics;
use ns_sema::analyze;

fn main() {
    // Путь к NetherScript-файлу, который нужно распарсить
    let path = "./from/main.ns";
    let parsed = match main_parse_with_diagnostics(path) {
        Ok(package) => package.into_merged_module(),
        Err(diagnostics) => {
            println!("");
            eprintln!("ns-parser: {} error(s)", diagnostics.len());
            println!("");
            for d in diagnostics {
                eprintln!("{}", d.render_pretty());
            }
            std::process::exit(1);
        }
    };
    match analyze(&parsed) {
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
