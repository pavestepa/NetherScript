use swc_common::{Globals, GLOBALS};

use crate::{ast::generate_ast::generate_ast, front_end::ast_to_er};

mod ast;
mod er;
mod front_end;
mod ir;
mod middle_end;

fn main() {
    // Путь к TypeScript-файлу, который нужно распарсить
    let path = "./from/main.ts";
    let (module, handler) = generate_ast(path);

    // Обрабатываем результат
    GLOBALS.set(&Globals::new(), || match module {
        Ok(m) => {
            println!("{:#?}", m);
        }
        Err(err) => {
            err.into_diagnostic(&handler).emit();
        }
    });

    ast_to_er(module);
}
