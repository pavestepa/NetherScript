mod data_type;
mod decl;
use crate::{
    semantics::hir::{self, Module},
    syntax::ast,
};
use decl::decl_mapper;

pub fn ast_mapper(from: ast::Module) -> hir::Module {
    let mut hir_decls = vec![];
    for decl in from.decls() {
        hir_decls.push(decl_mapper(decl));
    }

    Module { decls: hir_decls }
}
