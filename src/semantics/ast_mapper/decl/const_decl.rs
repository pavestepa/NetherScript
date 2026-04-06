use crate::{semantics::hir::{self, data_type}, syntax::ast};

pub fn const_decl_mapper(from: ast::ConstDecl) -> hir::decls::ConstDecl {
    hir::decls::ConstDecl {
        id: from.ident.0,
        data_type: 
        val:
    }
}
