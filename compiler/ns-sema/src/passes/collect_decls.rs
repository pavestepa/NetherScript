use ns_ast::{Decl, Ident, Module, ModuleDecl};

use crate::{
    context::SemaContext,
    symbol::SymbolKind,
};

pub fn run(ctx: &mut SemaContext, module: &Module) {
    for module_decl in module.decls() {
        collect_module_decl(ctx, module_decl);
    }
}

fn collect_module_decl(ctx: &mut SemaContext, module_decl: &ModuleDecl) {
    collect_decl(ctx, module_decl.decl());
}

fn collect_decl(ctx: &mut SemaContext, decl: &Decl) {
    match decl {
        Decl::Class(d) => declare_type(ctx, &d.ident, SymbolKind::Class),
        Decl::Interface(d) => declare_type(ctx, &d.ident, SymbolKind::Interface),
        Decl::Enum(d) => declare_type(ctx, &d.ident, SymbolKind::Enum),
        Decl::Function(d) => declare_value(ctx, &d.signature.ident, SymbolKind::Function),
        Decl::Type(d) => declare_type(ctx, &d.ident, SymbolKind::Type),
        Decl::Const(d) => declare_value(ctx, &d.binding.ident, SymbolKind::Value),
        Decl::TypeModifier(_) => {}
    }
}

fn declare_type(ctx: &mut SemaContext, ident: &Ident, kind: SymbolKind) {
    let name = ident_name(ident);
    let sid = ctx.intern_symbol(name.clone(), kind);
    if !ctx.declare_type_in_current_scope(&name, sid) {
        ctx.error(format!("duplicate type declaration: {name}"));
    }
}

fn declare_value(ctx: &mut SemaContext, ident: &Ident, kind: SymbolKind) {
    let name = ident_name(ident);
    let sid = ctx.intern_symbol(name.clone(), kind);
    if !ctx.declare_value_in_current_scope(&name, sid) {
        ctx.error(format!("duplicate value declaration: {name}"));
    }
}

fn ident_name(ident: &Ident) -> String {
    ident.clone().into_simple().as_str().to_string()
}
