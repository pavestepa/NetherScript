use ns_parser::main_parse;
use ns_sema::analyze;

fn main() {
    // Путь к NetherScript-файлу, который нужно распарсить
    let path = "./from/main.ns";
    let parsed = match main_parse(path) {
        Ok(package) => {
            let entry_path = package.entry_path().to_string();
            match package.into_entry_module() {
                Some(module) => module,
                None => {
                    eprintln!("ns-parser: entry module `{entry_path}` not found in package");
                    std::process::exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("ns-parser: {err}");
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
