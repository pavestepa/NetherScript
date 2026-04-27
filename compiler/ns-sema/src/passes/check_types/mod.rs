mod checker;
mod class_checks;
mod decls;
mod diagnostics;
mod enum_checks;
mod exprs;
mod interface_checks;
mod modifier_checks;
mod stmts;
mod type_system;
mod types;

use ns_ast::Module;

use crate::context::SemaContext;

pub fn run(ctx: &mut SemaContext, module: &Module) {
    let Some(builtins) = ctx.builtins.clone() else {
        ctx.error("internal error: builtin types are not seeded");
        return;
    };
    let mut checker = checker::TypeChecker::new(ctx, builtins, module);
    checker.check_module(module);
}
