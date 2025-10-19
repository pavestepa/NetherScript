use swc_ecma_ast::{Module, ModuleItem};

use crate::{er::module::ERModule, front_end::statmnt_gen};

pub fn ast_to_er(module: Module) -> ERModule {
    let mut er_module = ERModule { statmnts: vec![] };
    for stmt in module.body {
        let ast_stmt = stmt.as_mut_stmt();
        if ast_stmt.is_some() {
            er_module.statmnts.push(statmnt_gen(ast_stmt.unwrap()))
        }
    }
    return er_module;
}
