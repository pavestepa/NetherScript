use ns_ast::Decl;

use crate::ast_parser::Lowerer;
use crate::diagnostics::SirResult;

mod class_decl;
mod const_decl;
mod enum_decl;
mod function_decl;
mod type_decl;

pub(crate) fn lower_decl(lowerer: &mut Lowerer, decl: &Decl) -> SirResult<()> {
    match decl {
        Decl::Const(c) => const_decl::lower(lowerer, c),
        Decl::Function(f) => function_decl::lower(lowerer, f),
        Decl::Class(c) => class_decl::lower(lowerer, c),
        Decl::Enum(e) => enum_decl::lower(lowerer, e),
        Decl::Type(t) => type_decl::lower(lowerer, t),
        Decl::Error(_) => {
            lowerer.program.decls.push(crate::decl::SirDecl::Error);
            Ok(())
        }
        _ => {
            lowerer.program.decls.push(crate::decl::SirDecl::Error);
            Ok(())
        }
    }
}
